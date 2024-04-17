use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, thread};

use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, RedirectUrl, RefreshToken, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;

use crate::setting::{handle_setting_error, read_setting, write_setting, Setting};

#[derive(Debug, Deserialize)]
pub struct CalendarInfo {
    pub id: String,
    pub summary: String,
    pub background_color: String,
}

#[derive(Debug, Deserialize)]
struct CalendarInfoResponse {
    items: Vec<CalendarInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EventDateTime {
    pub date: Option<String>,
    pub date_time: Option<String>,
    pub time_zone: Option<String>,
}

impl fmt::Display for EventDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.date, &self.date_time) {
            (Some(date), _) => write!(f, "{}", date),
            (_, Some(date_time)) => write!(f, "{}", date_time),
            _ => write!(f, "No date or datetime specified"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EventInfo {
    pub id: String,
    pub summary: String,
    pub start: EventDateTime,
    pub end: EventDateTime,
    pub color_id: Option<String>,
    pub color: Option<String>,
    pub location: Option<String>,
}

#[derive(Deserialize)]
struct EventInfoResponse {
    items: Vec<EventInfo>,
}

/// Represents a Google Calendar client with authentication tokens.
///
/// This struct manages the authentication tokens required for accessing
/// Google Calendar APIs.
///
/// # Examples
///
/// ```
/// use crate::GoogleCalendar;
///
/// let mut google_calendar = GoogleCalendar::default();
///
/// // Call the get_calendar_list method
/// match google_calendar.get_calendar_list() {
///     Ok(calendar_list) => {
///         for calendar in calendar_list {
///             println!("{:?}", calendar);
///             match google_calendar
///                 .get_events_for_day_selected_calendar(&calendar.id, "2024-03-16")
///             {
///                 Ok(event_list) => {
///                     println!("Events for calendar '{}':", calendar.summary);
///                     for event in event_list {
///                         println!("{:?}", event);
///                     }
///                 }
///                 Err(err) => {
///                     eprintln!("Error retrieving events: {}", err);
///                 }
///             }
///         }
///     }
///     Err(err) => {
///         eprintln!("Error: {}", err);
///     }
/// }
/// ```
///
/// # Defaults
///
/// - `refresh_token`: An empty string by default.
/// - `port`: The default port number is set to 0.
/// - `access_token`: The access token is initialized as an empty string.
#[derive(Default)]
pub struct GoogleCalendar {
    /// The refresh token used to obtain new access tokens.
    refresh_token: String,
    /// The port number used for communication.
    port: u16,
    /// The access token used for authentication.
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

        let oauth_code: String;

        if let Err(err) = webbrowser::open(auth_url.as_str()) {
            eprintln!("Error opening browser: {:?}", err);
        }

        let (sender, receiver) = channel();

        // Spawn a thread to handle an incoming connection
        thread::spawn(move || match listener.accept() {
            Ok((stream, _)) => {
                if sender.send(Ok(stream)).is_err() {
                    eprintln!("Failed to send connection to main thread");
                }
            }
            Err(e) => {
                if sender.send(Err(e)).is_err() {
                    eprintln!("Failed to send error to main thread");
                }
            }
        });

        // Wait for the specified timeout duration or until a connection is received
        match receiver.recv_timeout(Duration::from_secs(120)) {
            Ok(Ok(stream)) => {
                oauth_code = handle_client(stream)?;
            }
            Ok(Err(e)) => return Err(e.into()),
            Err(_) => {
                //FIXME: resource may be leaked if handle an incoming connection thread runs forever.
                return Err(io::Error::new(io::ErrorKind::TimedOut, "Connection timed out").into());
            }
        }

        let refresh_token: RefreshToken = oauth2_client
            .exchange_code(AuthorizationCode::new(oauth_code.clone()))
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)?
            .refresh_token()
            .expect("Fail to get refresh token")
            .clone();

        // Assign the authorization code to the struct and setting.json
        self.refresh_token = refresh_token.secret().to_string();
        let _ = write_setting(
            Setting::GoogleRefreshToken,
            format!("\"{}\"", self.refresh_token).as_str(),
        );

        Ok(())
    }

    // Function to authorize the GoogleCalendar struct
    fn authorize(&mut self) -> Result<()> {
        let refresh_token: Option<String> = read_setting::<String>(Setting::GoogleRefreshToken)
            .unwrap_or_else(|err| {
                Some(handle_setting_error(
                    Setting::GoogleRefreshToken,
                    &err,
                    "Invalid Google authorization code".to_string(),
                ))
            });
        match refresh_token {
            Some(token) => {
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

                self.refresh_token = token;
                self.port = redirect_port;
                self.access_token = access_token;
                Ok(())
            }
            None => self.handle_authorization_code(),
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
        if self.refresh_token.is_empty() {
            self.authorize()?
        }

        if !self.is_access_token_valid() {
            let client = self.create_oauth2_client()?;

            let token_result = client
                .lock()
                .unwrap()
                .exchange_refresh_token(&RefreshToken::new(self.refresh_token.clone()))
                .request(http_client);

            match token_result {
                Ok(token) => {
                    // Update and stored access token

                    self.access_token = token.access_token().secret().to_string();

                    let _ = write_setting(
                        Setting::GoogleAccessToken,
                        &format!("\"{}\"", self.access_token),
                    );
                }

                Err(_) => {
                    //Regain the authorization code in case it's invalid. If error still occurs, return error.
                    self.handle_authorization_code()?;

                    let retry_token_result = client
                        .lock()
                        .unwrap()
                        .exchange_refresh_token(&RefreshToken::new(self.refresh_token.clone()))
                        .request(http_client)?;

                    self.access_token = retry_token_result.access_token().secret().to_string();

                    let _ = write_setting(
                        Setting::GoogleAccessToken,
                        &format!("\"{}\"", self.access_token),
                    );
                }
            }
        }

        Ok(self.access_token.clone())
    }

    // Function to get the calendar list using the access token
    pub fn get_calendar_list(&mut self) -> Result<Vec<CalendarInfo>> {
        let api_url = "https://www.googleapis.com/calendar/v3/users/me/calendarList?fields=items(id%2Csummary%2CbackgroundColor)";
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
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}T00:00:00Z&timeMax={}T23:59:59Z&fields=items(id,summary,start(date,dateTime,timeZone),end(date,dateTime,timeZone),colorId,location)",
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
            let mut event_list: EventInfoResponse = response.json()?;

            // Iterate over each event in the event_list
            for event in &mut event_list.items {
                if let Some(color_id) = event.color_id.clone() {
                    // Get the color code for the color_id
                    if let Some(color_code) = get_color_code(&color_id) {
                        event.color = Some(color_code.to_string());
                    } else {
                        eprintln!("Color code not found for color_id: {}", color_id);
                    }
                }
            }
            Ok(event_list.items)
        } else {
            Err(anyhow::anyhow!("Error calling Google Calendar API"))
        }
    }
}

// Function to replace hash with percent_23 in a string
fn replace_hash_with_percent_23(input_string: &str) -> String {
    input_string.replace("#", "%23")
}

// return the representing the corresponding color code from Google Event Color Id
fn get_color_code(color_id: &str) -> Option<&str> {
    let color_palette: HashMap<&str, &str> = [
        ("1", "#a4bdfc"),  // Blue
        ("2", "#7ae7bf"),  // Green
        ("3", "#dbadff"),  // Purple
        ("4", "#ff887c"),  // Red
        ("5", "#fbd75b"),  // Yellow
        ("6", "#ffb878"),  // Orange
        ("7", "#46d6db"),  // Turquoise
        ("8", "#e1e1e1"),  // Gray
        ("9", "#5484ed"),  // Bold Blue
        ("10", "#51b749"), // Bold Green
        ("11", "#dc2127"), // Bold Red
        ("12", "#dbadff"), // Bold Purple
                           // Add more colors if needed
    ]
    .iter()
    .cloned()
    .collect();

    color_palette.get(color_id).copied()
}
