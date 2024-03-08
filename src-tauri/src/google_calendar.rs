use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
pub struct CalendarInfo {
    pub id: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
struct CalendarInfoResponse {
    items: Vec<CalendarInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EventInfo {
    pub id: String,
    pub summary: String,
}

#[derive(Deserialize)]
struct EventInfoResponse {
    items: Vec<EventInfo>,
}

#[derive(Default)]
pub struct GoogleCalendar {
    pkce_verifier: String,
    authorization_code: String,
    port: u16,
    access_token: String,
}

impl GoogleCalendar {
    // Function to create the OAuth2 client
    fn create_oauth2_client(&self) -> Result<Arc<Mutex<BasicClient>>> {
        let client_id =
            "68238066373-iaiepboevrqvu1q6hp7gcirvro06rgvg.apps.googleusercontent.com".to_string();
        let client_secret = "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".to_string();
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
        let token_url = TokenUrl::new("https://accounts.google.com/o/oauth2/token".to_string())?;

        let redirect_url = format!("http://localhost:{}", self.port).to_string(); // Use a placeholder for now

        let oauth2_client = Arc::new(Mutex::new(
            BasicClient::new(
                oauth2::ClientId::new(client_id),
                Some(oauth2::ClientSecret::new(client_secret)),
                auth_url,
                Some(token_url),
            )
            .set_redirect_uri(RedirectUrl::new(redirect_url)?),
        ));

        Ok(oauth2_client)
    }

    // Function to handle the authorization code
    async fn handle_authorization_code(&mut self) -> Result<()> {
        async fn handle_client(stream: tokio::net::TcpStream) -> Result<String> {
            let mut buffer = String::new();
            let mut reader = BufReader::new(stream);

            tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut buffer).await?;

            let code = extract_code_from_response(&buffer);

            let success_response = r#"HTTP/1.1 200 OK
            Content-Type: text/html

            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Login Success</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        background-color: #f4f4f4;
                        text-align: center;
                        margin: 20px;
                    }
                    h1 {
                        color: #2ecc71;
                    }
                </style>
            </head>
            <body>
                <h1>Login Successful!</h1>
                <p>Thank you for authorizing the application.</p>
            </body>
            </html>"#;

            let _ = reader.write_all(success_response.as_bytes()).await;

            let _ = reader.shutdown().await;

            Ok(code)
        }

        fn extract_code_from_response(response: &str) -> String {
            if let Some(start_idx) = response.find("code=") {
                let start_idx = start_idx + 5; // Move past "code="
                if let Some(end_idx) = response[start_idx..].find('&') {
                    return response[start_idx..start_idx + end_idx].to_string();
                } else {
                    return response[start_idx..].to_string();
                }
            }
            String::new()
        }

        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        self.pkce_verifier = pkce_verifier.secret().clone();

        // Start a local HTTP server to listen for the redirect URI
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let local_addr = listener.local_addr()?;
        let port = local_addr.port();
        self.port = port;

        let redirect_url = format!("http://localhost:{}", port);

        // Set the redirect URI for the OAuth2 client
        let oauth2_client = self
            .create_oauth2_client()?
            .lock()
            .unwrap()
            .clone()
            .set_redirect_uri(RedirectUrl::new(redirect_url)?);

        // Generate authorization URL
        let (auth_url, _) = oauth2_client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(vec![Scope::new(
                "https://www.googleapis.com/auth/calendar.readonly".to_string(),
            )])
            .set_pkce_challenge(pkce_challenge)
            .url();

        let mut code = String::new();

        if let Err(err) = webbrowser::open(auth_url.as_str()) {
            eprintln!("Error opening browser: {:?}", err);
        }

        // Accept the first incoming connection
        if let Ok((stream, _)) = listener.accept().await {
            code = handle_client(stream).await?;
        }

        // Assign the authorization code to the struct
        self.authorization_code = code;

        Ok(())
    }

    // Function to authorize the GoogleCalendar struct
    pub async fn authorize(&mut self) {
        self.handle_authorization_code().await.unwrap();
    }

    async fn is_access_token_valid(&self) -> bool {
        let validation_url = "https://www.googleapis.com/oauth2/v1/tokeninfo";
        let access_token = &self.access_token;

        let response = reqwest::Client::new()
            .get(validation_url)
            .query(&[("access_token", access_token)])
            .send()
            .await;

        match response {
            Ok(response) if response.status().as_u16() == 200 => true,
            _ => false,
        }
    }

    // Function to get the access token using the authorization code and OAuth2 client
    async fn get_access_token(&mut self) -> Result<String> {
        if !self.is_access_token_valid().await {
            let code = &self.authorization_code;
            let pkce_verifier = PkceCodeVerifier::new(self.pkce_verifier.clone());

            let token_result = self
                .create_oauth2_client()?
                .lock()
                .unwrap()
                .exchange_code(AuthorizationCode::new(code.clone()))
                .set_pkce_verifier(pkce_verifier)
                .request_async(async_http_client)
                .await
                .map_err(|err| {
                    // You can customize this error conversion based on your needs
                    anyhow::anyhow!("Error exchanging authorization code: {:?}", err)
                })?;

            // Update the stored access token
            self.access_token = token_result.access_token().secret().to_string();
        }

        Ok(self.access_token.clone())
    }

    // Function to get the calendar list using the access token
    pub async fn get_calendar_list(&mut self) -> Result<Vec<CalendarInfo>> {
        let api_url = "https://www.googleapis.com/calendar/v3/users/me/calendarList?fields=items(id%2Csummary)";
        let access_token = self.get_access_token().await?;

        let response = reqwest::Client::new()
            .get(api_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let calendar_list: CalendarInfoResponse = response.json().await?;
            Ok(calendar_list.items)
        } else {
            println!("Error calling Google Calendar API: {:?}", response.status());
            Err(anyhow::anyhow!("Error calling Google Calendar API"))
        }
    }

    // Function to get events for a specific day for the provided calendar ID
    pub async fn get_events_for_day_selected_calendar(
        &mut self,
        calendar_id: &str,
        date: &str,
    ) -> Result<Vec<EventInfo>> {
        let sanitized_calendar_id = replace_hash_with_percent_23(calendar_id);

        let api_url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}T00:00:00Z&timeMax={}T23:59:59Z&fields=items(id,summary,start,end)",
            sanitized_calendar_id, date, date
        );

        let access_token = self.get_access_token().await?;

        let response = reqwest::Client::new()
            .get(&api_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let event_list: EventInfoResponse = response.json().await?;
            Ok(event_list.items)
        } else {
            println!(
                "Error calling Google Calendar API: {:?} \n {:?}",
                response.status(),
                response.error_for_status()
            );
            Err(anyhow::anyhow!("Error calling Google Calendar API"))
        }
    }
}

// Function to replace hash with percent_23 in a string
fn replace_hash_with_percent_23(input_string: &str) -> String {
    let result_string = input_string.replace("#", "%23");
    result_string
}
