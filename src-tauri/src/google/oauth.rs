use log::info;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn authorize() {
    let auth_url = "http://localhost:8080/auth/google";
    webbrowser::open(auth_url).unwrap();
}

// Listen for the auth callback on the specified port
pub fn handle_auth_callback() {
    let listener = TcpListener::bind("127.0.0.1:25794").unwrap();
    info!("Waiting for auth callback...");

    let mut stream = listener.accept().unwrap().0;
    handle_connection(&mut stream);
}

fn handle_connection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

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
    let _ = stream.shutdown(std::net::Shutdown::Both);

    println!("Request: {http_request:#?}");
}

// impl GoogleOAuth {
//     // Function to create the OAuth2 client
//     pub fn create_oauth2_client(&self) -> Result<Arc<Mutex<BasicClient>>> {
//         let conf = config::oauth::get_oauth_config();

//         let client_secret = conf.client_secret.clone();
//         let auth_url = AuthUrl::new(conf.auth_url.clone())?;
//         let token_url = TokenUrl::new(conf.token_url.clone())?;
//         let redirect_url = format!("http://localhost:{}", self.port).to_string();

//         let oauth2_client = Arc::new(Mutex::new(
//             BasicClient::new(
//                 oauth2::ClientId::new(conf.client_id.clone()),
//                 Some(oauth2::ClientSecret::new(client_secret)),
//                 auth_url,
//                 Some(token_url),
//             )
//             .set_redirect_uri(RedirectUrl::new(redirect_url)?),
//         ));

//         Ok(oauth2_client)
//     }

//     // Function to handle the authorization code
//     pub fn handle_authorization_code(&mut self) -> Result<()> {
//         fn handle_client(mut stream: TcpStream) -> Result<String, std::io::Error> {
//             let mut buffer = String::new();

//             // Read from the stream until finding the authorization code or an error occurs
//             loop {
//                 let mut byte = [0; 1];
//                 match stream.read_exact(&mut byte) {
//                     Ok(_) => {
//                         buffer.push(byte[0] as char);
//                         if buffer.ends_with("\r\n\r\n") {
//                             break; // Stop reading when reaching the end of headers
//                         }
//                     }
//                     Err(e) => return Err(e),
//                 }
//             }

//             let code = extract_code_from_response(&buffer);

//             let success_response = r#"HTTP/1.1 200 OK
//         Content-Type: text/html

//         <!DOCTYPE html>
//         <html lang="en">
//         <head>
//             <meta charset="UTF-8">
//             <meta name="viewport" content="width=device-width, initial-scale=1.0">
//             <title>Login Success</title>
//             <style>
//                 body {
//                     font-family: Arial, sans-serif;
//                     background-color: #f4f4f4;
//                     text-align: center;
//                     margin: 20px;
//                 }
//                 h1 {
//                     color: #2ecc71;
//                 }
//             </style>
//         </head>
//         <body>
//             <h1>Login Successful!</h1>
//             <p>Thank you for authorizing the application.</p>
//         </body>
//         </html>"#;

//             let _ = stream.write_all(success_response.as_bytes());

//             // Close the connection
//             let _ = stream.shutdown(std::net::Shutdown::Both);

//             Ok(code)
//         }
//         fn extract_code_from_response(response: &str) -> String {
//             if let Some(start_idx) = response.find("code=") {
//                 let start_idx = start_idx + 5; // Move past "code="
//                 if let Some(end_idx) = response[start_idx..].find('&') {
//                     return response[start_idx..start_idx + end_idx].to_string();
//                 } else {
//                     return response[start_idx..].to_string();
//                 }
//             }
//             String::new()
//         }

//         // Generate PKCE challenge
//         let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

//         // Start a local HTTP server to listen for the redirect URI
//         let listener = TcpListener::bind("127.0.0.1:0")?;
//         let local_addr = listener.local_addr()?;
//         let port = local_addr.port();
//         self.port = port;

//         let _ = write_setting(SettingName::RedirectPort, &port.to_string());

//         let redirect_url = format!("http://localhost:{}", port);

//         // Set the redirect URI for the OAuth2 client
//         let oauth2_client = self
//             .create_oauth2_client()?
//             .lock()
//             .unwrap()
//             .clone()
//             .set_redirect_uri(RedirectUrl::new(redirect_url)?);

//         // Generate authorization URL
//         let (auth_url, _) = oauth2_client
//             .authorize_url(CsrfToken::new_random)
//             .add_scopes(vec![Scope::new(
//                 "https://www.googleapis.com/auth/calendar.readonly".to_string(),
//             )])
//             .set_pkce_challenge(pkce_challenge)
//             .url();

//         let oauth_code: String;

//         if let Err(err) = webbrowser::open(auth_url.as_str()) {
//             error!("Error opening browser: {:?}", err);
//         }

//         let (sender, receiver) = channel();

//         // Spawn a thread to handle an incoming connection
//         thread::spawn(move || match listener.accept() {
//             Ok((stream, _)) => {
//                 if sender.send(Ok(stream)).is_err() {
//                     eprintln!("Failed to send connection to main thread");
//                 }
//             }
//             Err(e) => {
//                 if sender.send(Err(e)).is_err() {
//                     eprintln!("Failed to send error to main thread");
//                 }
//             }
//         });

//         // Wait for the specified timeout duration or until a connection is received
//         match receiver.recv_timeout(Duration::from_secs(120)) {
//             Ok(Ok(stream)) => {
//                 oauth_code = handle_client(stream)?;
//             }
//             Ok(Err(e)) => return Err(e.into()),
//             Err(_) => {
//                 //FIXME: resource may be leaked if handle an incoming connection thread runs forever.
//                 return Err(io::Error::new(io::ErrorKind::TimedOut, "Connection timed out").into());
//             }
//         }

//         let refresh_token: RefreshToken = oauth2_client
//             .exchange_code(AuthorizationCode::new(oauth_code.clone()))
//             .set_pkce_verifier(pkce_verifier)
//             .request(http_client)?
//             .refresh_token()
//             .expect("Fail to get refresh token")
//             .clone();

//         // Assign the authorization code to the struct and setting.json
//         self.refresh_token = refresh_token.secret().to_string();
//         let _ = write_setting(
//             SettingName::GoogleRefreshToken,
//             format!("\"{}\"", self.refresh_token).as_str(),
//         );

//         Ok(())
//     }

//     // Function to authorize the GoogleCalendar struct
//     pub fn authorize(&mut self) -> Result<()> {
//         let refresh_token: Option<String> = read_setting::<String>(SettingName::GoogleRefreshToken)
//             .unwrap_or_else(|err| {
//                 Some(handle_setting_error(
//                     SettingName::GoogleRefreshToken,
//                     &err,
//                     "Invalid Google authorization code".to_string(),
//                 ))
//             });
//         match refresh_token {
//             Some(token) => {
//                 let redirect_port: u16 = read_setting::<u16>(SettingName::RedirectPort)
//                     .unwrap_or_else(|err| {
//                         Some(handle_setting_error(SettingName::RedirectPort, &err, 0))
//                     })
//                     .unwrap_or_default();

//                 let access_token: String = read_setting::<String>(SettingName::GoogleAccessToken)
//                     .unwrap_or_else(|err| {
//                         Some(handle_setting_error(
//                             SettingName::GoogleAccessToken,
//                             &err,
//                             "Invalid Google Access Token".into(),
//                         ))
//                     })
//                     .unwrap_or_default();

//                 self.refresh_token = token;
//                 self.port = redirect_port;
//                 self.access_token = access_token;
//                 Ok(())
//             }
//             None => self.handle_authorization_code(),
//         }
//     }

//     pub fn is_access_token_valid(&self) -> bool {
//         let validation_url = "https://www.googleapis.com/oauth2/v1/tokeninfo";
//         let access_token = &self.access_token;

//         let response =
//             reqwest::blocking::get(format!("{}?access_token={}", validation_url, access_token));

//         match response {
//             Ok(response) if response.status().as_u16() == 200 => true,
//             _ => false,
//         }
//     }

//     // Function to get the access token using the authorization code and OAuth2 client
//     pub fn get_access_token(&mut self) -> Result<String> {
//         if self.refresh_token.is_empty() {
//             self.authorize()?
//         }

//         if !self.is_access_token_valid() {
//             let client = self.create_oauth2_client()?;

//             let token_result = client
//                 .lock()
//                 .unwrap()
//                 .exchange_refresh_token(&RefreshToken::new(self.refresh_token.clone()))
//                 .request(http_client);

//             match token_result {
//                 Ok(token) => {
//                     // Update and stored access token

//                     self.access_token = token.access_token().secret().to_string();

//                     let _ = write_setting(
//                         SettingName::GoogleAccessToken,
//                         &format!("\"{}\"", self.access_token),
//                     );
//                 }

//                 Err(_) => {
//                     //Regain the authorization code in case it's invalid. If error still occurs, return error.
//                     self.handle_authorization_code()?;

//                     let retry_token_result = client
//                         .lock()
//                         .unwrap()
//                         .exchange_refresh_token(&RefreshToken::new(self.refresh_token.clone()))
//                         .request(http_client)?;

//                     self.access_token = retry_token_result.access_token().secret().to_string();

//                     let _ = write_setting(
//                         SettingName::GoogleAccessToken,
//                         &format!("\"{}\"", self.access_token),
//                     );
//                 }
//             }
//         }

//         Ok(self.access_token.clone())
//     }
// }
