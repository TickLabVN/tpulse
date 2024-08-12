use crate::config::{self, GoogleSetting};
use log::info;
use reqwest::header::CONTENT_TYPE;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const AUTH_URL: &str = "http://localhost:8080";

#[tauri::command]
pub fn connect_google_account() {
    let auth_url = format!("{}/auth/google", AUTH_URL);
    webbrowser::open(auth_url.as_str()).unwrap();

    handle_auth_callback();
}

pub fn refresh_token() {
    let tokens: GoogleSetting = {
        let setting = config::get_setting();
        let refresh_token = setting.google.as_ref().unwrap().refresh_token.clone();

        let url = format!("{}/auth/google/token/refresh", AUTH_URL);
        let client = reqwest::blocking::Client::new();

        let body = format!("{{\"refreshToken\":\"{}\"}}", refresh_token);
        let response = client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .unwrap();

        response.json().unwrap()
    };

    let mut setting = config::get_mutable_setting();
    setting.google = Some(tokens);
    config::save_setting(&setting);
}

// Listen for the auth callback on the specified port
fn handle_auth_callback() {
    let listener = TcpListener::bind("127.0.0.1:25794").unwrap();

    info!("Waiting for auth callback...");
    let mut stream = listener.accept().unwrap().0;

    handle_connection(&mut stream);
}

fn handle_connection(mut stream: &TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    // GET /?uuid=f09c46c3-82ef-4f47-9752-bbf4cd9503f8 HTTP/1.1
    let mut method = String::new();
    buf_reader.read_line(&mut method).unwrap();

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

    // GET /?uuid=f09c46c3-82ef-4f47-9752-bbf4cd9503f8 HTTP/1.1
    let uuid = &method[11..47];
    fetch_google_tokens(uuid);
}

fn fetch_google_tokens(uuid: &str) {
    // Fetch the tokens from the server
    let url = format!("{}/auth/google/token?uuid={}", AUTH_URL, uuid);
    let response = reqwest::blocking::get(&url).unwrap();
    let tokens: GoogleSetting = response.json().unwrap();

    let mut setting = config::get_mutable_setting();
    setting.google = Some(tokens);
    config::save_setting(&setting);
}
