use crate::{config, db};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use rusqlite::params;
use serde::Deserialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::refresh_token;

#[derive(Deserialize, Debug)]
struct CalendarTime {
    #[serde(rename = "dateTime")]
    date_time: String,
}

impl CalendarTime {
    fn to_unix_secs(&self) -> i64 {
        let datetime = OffsetDateTime::parse(&self.date_time, &Rfc3339).unwrap();
        datetime.unix_timestamp()
    }
}

#[derive(Deserialize, Debug)]
struct CalendarEventItem {
    summary: Option<String>,
    description: Option<String>,
    start: CalendarTime,
    status: String,
    end: CalendarTime,
    id: String,
}

#[derive(Deserialize, Debug)]
struct UnauthenticatedError {
    status: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GoogleCalendarResp {
    Error { error: UnauthenticatedError },
    Data { items: Vec<CalendarEventItem> },
}

#[tauri::command]
pub fn sync_google_calendar(from_date: &str, to_date: &str) -> bool {
    let response = fetch_events(from_date, to_date);
    if response.is_none() {
        return false;
    }
    let response = response.unwrap();
    let resp_body = response.text().unwrap();
    let error_code = match serde_json::from_str::<GoogleCalendarResp>(&resp_body).unwrap() {
        GoogleCalendarResp::Error { error } => {
            if error.status == "UNAUTHENTICATED" {
                1
            } else {
                log::error!("[Fetch google calendar event] {:?}", error);
                2
            }
        }
        GoogleCalendarResp::Data { items } => {
            save_events(&items);
            0
        }
    };

    if error_code == 0 {
        // Fetch items successfully
        true
    } else if error_code == 1 {
        // UNAUTHENTICATED, need to refresh token
        refresh_token();
        sync_google_calendar(from_date, to_date)
    } else {
        // Other error code. Need to login google first
        false
    }
}

fn save_events(items: &Vec<CalendarEventItem>) {
    let mut conn = db::get_connection();
    let tx = conn.transaction().expect("Failed to start transaction");

    for item in items {
        let start_time = item.start.to_unix_secs();
        let end_time = item.end.to_unix_secs();

        let summary = item.summary.clone().unwrap_or("Untitled".to_string());
        if item.status == "cancelled" {
            tx.execute(
                "DELETE FROM plan WHERE source = 'google' AND external_id = ?1",
                params![&item.id],
            )
            .unwrap();
        } else {
            tx.execute(
                "INSERT INTO plan (name, description, start_time, end_time, external_id , source)
                    VALUES (?1, ?2, ?3, ?4, ?5, 'google')
                    ON CONFLICT(source, external_id) 
                    DO UPDATE SET name = ?1, description = ?2, start_time = ?3, end_time = ?4",
                params![
                    &summary,
                    &item.description,
                    start_time,
                    end_time,
                    &item.id
                ],
            )
            .unwrap();
        }
    }

    tx.commit().expect("Failed to commit transaction");
}

fn fetch_events(from_date: &str, to_date: &str) -> Option<Response> {
    let setting = config::get_setting();

    let google_setting = &setting.google;
    if google_setting.is_none() {
        return None;
    }

    let google_setting = google_setting.clone().unwrap();
    let access_token = google_setting.access_token;

    let url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/primary/events?timeMin={}&timeMax={}&showDeleted=true&singleEvents=true",
        from_date, to_date
    );

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );

    let resp = client.get(&url).headers(headers).send().unwrap();
    Some(resp)
}
