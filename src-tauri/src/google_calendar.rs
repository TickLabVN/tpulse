use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;

use crate::setting::{handle_setting_error, read_setting, write_setting, Setting};

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
    fn handle_authorization_code(&mut self) -> Result<()> {
        fn handle_client(mut stream: TcpStream) -> Result<String, std::io::Error> {
            let mut buffer = String::new();

            // Read from the stream until finding the authorization code or an error occurs
            loop {
                let mut byte = [0; 1];
                match stream.read_exact(&mut byte) {
                    Ok(_) => {
                        buffer.push(byte[0] as char);
                        if buffer.ends_with("\r\n\r\n") {
                            break; // Stop reading when reaching the end of headers
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

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

            let _ = stream.write_all(success_response.as_bytes());

            // Close the connection
            let _ = stream.shutdown(std::net::Shutdown::Both);

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
        let _ = write_setting(
            Setting::PkceVerifier,
            format!("\"{}\"", self.pkce_verifier).as_str(),
        );

        // Start a local HTTP server to listen for the redirect URI
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let local_addr = listener.local_addr()?;
        let port = local_addr.port();
        self.port = port;

        let _ = write_setting(Setting::RedirectPort, &port.to_string());

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
        if let Ok((stream, _)) = listener.accept() {
            code = handle_client(stream)?;
        }

        // Assign the authorization code to the struct and setting.json
        self.authorization_code = code.clone();
        let _ = write_setting(
            Setting::GoogleAuthorizationCode,
            format!("\"{}\"", code).as_str(),
        );

        Ok(())
    }

    // Function to authorize the GoogleCalendar struct
    fn authorize(&mut self) {
        let oauth_code: Option<String> = read_setting::<String>(Setting::GoogleAuthorizationCode)
            .unwrap_or_else(|err| {
                Some(handle_setting_error(
                    Setting::GoogleAuthorizationCode,
                    &err,
                    "Invalid Google authorization code".to_string(),
                ))
            });
        match oauth_code {
            Some(code) => {
                let pkce_verifier: String = read_setting::<String>(Setting::PkceVerifier)
                    .unwrap_or_else(|err| {
                        Some(handle_setting_error(
                            Setting::PkceVerifier,
                            &err,
                            "Invalid Pkce Verifier secret".to_string(),
                        ))
                    })
                    .unwrap_or_default();

                let redirect_port: u16 = read_setting::<u16>(Setting::RedirectPort)
                    .unwrap_or_else(|err| {
                        Some(handle_setting_error(Setting::RedirectPort, &err, 0))
                    })
                    .unwrap_or_default();

                let access_token: String = read_setting::<String>(Setting::GoogleAccessToken)
                    .unwrap_or_else(|err| {
                        Some(handle_setting_error(
                            Setting::GoogleAccessToken,
                            &err,
                            "Invalid Google Access Token".into(),
                        ))
                    })
                    .unwrap_or_default();

                self.authorization_code = code;
                self.pkce_verifier = pkce_verifier;
                self.port = redirect_port;
                self.access_token = access_token;
            }
            None => {
                self.handle_authorization_code().unwrap();
            }
        }
    }

    fn is_access_token_valid(&self) -> bool {
        let validation_url = "https://www.googleapis.com/oauth2/v1/tokeninfo";
        let access_token = &self.access_token;

        let response =
            reqwest::blocking::get(format!("{}?access_token={}", validation_url, access_token));

        match response {
            Ok(response) if response.status().as_u16() == 200 => true,
            _ => false,
        }
    }

    // Function to get the access token using the authorization code and OAuth2 client
    fn get_access_token(&mut self) -> Result<String> {
        if self.authorization_code.is_empty() {
            self.authorize();
        }

        if !self.is_access_token_valid() {
            let code = &self.authorization_code;
            let pkce_verifier = PkceCodeVerifier::new(self.pkce_verifier.clone());

            let token_result = self
                .create_oauth2_client()?
                .lock()
                .unwrap()
                .exchange_code(AuthorizationCode::new(code.clone()))
                .set_pkce_verifier(pkce_verifier)
                .request(http_client)?;

            // Update the stored access token
            self.access_token = token_result.access_token().secret().to_string();
            let _ = write_setting(
                Setting::GoogleAccessToken,
                &format!("\"{}\"", self.access_token),
            );
        }

        Ok(self.access_token.clone())
    }

    // Function to get the calendar list using the access token
    pub fn get_calendar_list(&mut self) -> Result<Vec<CalendarInfo>> {
        let api_url = "https://www.googleapis.com/calendar/v3/users/me/calendarList?fields=items(id%2Csummary)";
        let access_token = self.get_access_token()?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
        );

        let client = Client::new();
        let response = client.get(api_url).headers(headers).send()?;

        if response.status().is_success() {
            let calendar_list: CalendarInfoResponse = response.json()?;
            Ok(calendar_list.items)
        } else {
            println!("Error calling Google Calendar API: {:?}", response.status());
            Err(anyhow::anyhow!("Error calling Google Calendar API"))
        }
    }

    // Function to get events for a specific day for the provided calendar ID
    pub fn get_events_for_day_selected_calendar(
        &mut self,
        calendar_id: &str,
        date: &str,
    ) -> Result<Vec<EventInfo>> {
        let sanitized_calendar_id = replace_hash_with_percent_23(calendar_id);

        let api_url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}T00:00:00Z&timeMax={}T23:59:59Z&fields=items(id,summary,start,end)",
            sanitized_calendar_id, date, date
        );

        let access_token = self.get_access_token()?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
        );

        let client = Client::new();
        let response = client.get(api_url).headers(headers).send()?;

        if response.status().is_success() {
            let event_list: EventInfoResponse = response.json()?;
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
    input_string.replace("#", "%23")
}
