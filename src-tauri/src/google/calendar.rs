// use anyhow::Result;
// use log::{error, info};
// use reqwest::blocking::Client;
// use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fmt;

// use super::GoogleOAuth;

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CalendarInfo {
//     pub id: String,
//     pub summary: String,
//     pub background_color: String,
// }

// #[derive(Debug, Deserialize)]
// struct CalendarInfoResponse {
//     items: Vec<CalendarInfo>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct EventDateTime {
//     pub date: Option<String>,
//     pub date_time: Option<String>,
//     pub time_zone: Option<String>,
// }

// impl fmt::Display for EventDateTime {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match (&self.date, &self.date_time) {
//             (Some(date), _) => write!(f, "{}", date),
//             (_, Some(date_time)) => write!(f, "{}", date_time),
//             _ => write!(f, "No date or datetime specified"),
//         }
//     }
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct EventInfo {
//     pub id: String,
//     pub summary: String,
//     pub start: EventDateTime,
//     pub end: EventDateTime,
//     pub color_id: Option<String>,
//     pub color: Option<String>,
//     pub location: Option<String>,
// }

// #[derive(Deserialize)]
// struct EventInfoResponse {
//     items: Vec<EventInfo>,
// }

// /// Represents a Google Calendar client with authentication tokens.
// ///
// /// This struct manages the authentication tokens required for accessing
// /// Google Calendar APIs.
// ///
// /// # Examples
// ///
// /// ```
// /// use tpulse::google_calendar::GoogleCalendar;
// ///
// /// let mut google_calendar = GoogleCalendar::default();
// ///
// /// // Call the get_calendar_list method
// /// match google_calendar.get_calendar_list() {
// ///     Ok(calendar_list) => {
// ///         for calendar in calendar_list {
// ///             println!("{:?}", calendar);
// ///             match google_calendar
// ///                 .get_events_for_day_selected_calendar(&calendar.id, "2024-03-16")
// ///             {
// ///                 Ok(event_list) => {
// ///                     println!("Events for calendar '{}':", calendar.summary);
// ///                     for event in event_list {
// ///                         println!("{:?}", event);
// ///                     }
// ///                 }
// ///                 Err(err) => {
// ///                     eprintln!("Error retrieving events: {}", err);
// ///                 }
// ///             }
// ///         }
// ///     }
// ///     Err(err) => {
// ///         eprintln!("Error: {}", err);
// ///     }
// /// }
// /// ```
// ///
// /// # Defaults
// ///
// /// - `refresh_token`: An empty string by default.
// /// - `port`: The default port number is set to 0.
// /// - `access_token`: The access token is initialized as an empty string.
// #[derive(Default)]
// pub struct GoogleCalendar {}

// impl GoogleCalendar {
//     // Function to get the calendar list using the access token
//     pub fn get_calendar_list(&mut self) -> Result<Vec<CalendarInfo>> {
//         let mut google_oauth = GoogleOAuth::default();
//         let api_url = "https://www.googleapis.com/calendar/v3/users/me/calendarList?fields=items(id%2Csummary%2CbackgroundColor)";
//         let access_token = google_oauth.get_access_token()?;

//         let mut headers = HeaderMap::new();
//         headers.insert(
//             AUTHORIZATION,
//             HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
//         );

//         let client = Client::new();
//         let response = client.get(api_url).headers(headers).send()?;

//         if response.status().is_success() {
//             let calendar_list: CalendarInfoResponse = response.json()?;
//             Ok(calendar_list.items)
//         } else {
//             Err(anyhow::anyhow!("Error calling Google Calendar API"))
//         }
//     }

//     // Function to get events for a specific day for the provided calendar ID
//     pub fn get_events_for_day_selected_calendar(
//         &mut self,
//         calendar_id: &str,
//         date: &str,
//     ) -> Result<Vec<EventInfo>> {
//         let mut google_oauth = GoogleOAuth::default();
//         let sanitized_calendar_id = replace_hash_with_percent_23(calendar_id);

//         let api_url = format!(
//             "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}T00:00:00Z&timeMax={}T23:59:59Z&fields=items(id,summary,start(date,dateTime,timeZone),end(date,dateTime,timeZone),colorId,location)",
//             sanitized_calendar_id, date, date
//         );

//         let access_token = google_oauth.get_access_token()?;

//         let mut headers = HeaderMap::new();
//         headers.insert(
//             AUTHORIZATION,
//             HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
//         );

//         let client = Client::new();
//         let response = client.get(api_url).headers(headers).send()?;

//         if response.status().is_success() {
//             let mut event_list: EventInfoResponse = response.json()?;

//             // Iterate over each event in the event_list
//             for event in &mut event_list.items {
//                 if let Some(color_id) = event.color_id.clone() {
//                     // Get the color code for the color_id
//                     if let Some(color_code) = get_color_code(&color_id) {
//                         event.color = Some(color_code.to_string());
//                     } else {
//                         error!("Color code not found for color_id: {}", color_id);
//                     }
//                 }
//             }
//             Ok(event_list.items)
//         } else {
//             Err(anyhow::anyhow!("Error calling Google Calendar API"))
//         }
//     }
// }

// // Function to replace hash with percent_23 in a string
// fn replace_hash_with_percent_23(input_string: &str) -> String {
//     input_string.replace("#", "%23")
// }

// // return the representing the corresponding color code from Google Event Color Id
// fn get_color_code(color_id: &str) -> Option<&str> {
//     let color_palette: HashMap<&str, &str> = [
//         ("1", "#a4bdfc"),  // Blue
//         ("2", "#7ae7bf"),  // Green
//         ("3", "#dbadff"),  // Purple
//         ("4", "#ff887c"),  // Red
//         ("5", "#fbd75b"),  // Yellow
//         ("6", "#ffb878"),  // Orange
//         ("7", "#46d6db"),  // Turquoise
//         ("8", "#e1e1e1"),  // Gray
//         ("9", "#5484ed"),  // Bold Blue
//         ("10", "#51b749"), // Bold Green
//         ("11", "#dc2127"), // Bold Red
//         ("12", "#dbadff"), // Bold Purple
//                            // Add more colors if needed
//     ]
//     .iter()
//     .cloned()
//     .collect();

//     color_palette.get(color_id).copied()
// }

// #[tauri::command]
// pub fn handle_google_calendar(date: String) -> Result<String, String> {
//     let mut google_calendar = GoogleCalendar::default();
//     let mut calendar_infos: Vec<EventInfo> = Vec::new();

//     match google_calendar.get_calendar_list() {
//         Ok(calendar_list) => {
//             for calendar in calendar_list {
//                 let events = match google_calendar
//                     .get_events_for_day_selected_calendar(&calendar.id, &date)
//                 {
//                     Ok(events) => events,
//                     Err(err) => {
//                         error!("Error getting events: {}", err);
//                         continue; // Continue to the next calendar if there's an error
//                     }
//                 };
//                 calendar_infos.extend(events);
//                 use std::io::{self, Write};

//                 info!("{:?}", calendar_infos);

//                 print!("Press Enter to continue...");
//                 io::stdout().flush().unwrap();
//             }
//             // Serialize the vector of EventInfo into a JSON string
//             match serde_json::to_string(&calendar_infos) {
//                 Ok(json_string) => Ok(json_string),
//                 Err(err) => Err(format!("Error serializing JSON: {}", err)),
//             }
//         }
//         Err(err) => {
//             // Create an error message string
//             let error_msg = format!("Error getting calendar list: {}", err);
//             Err(error_msg)
//         }
//     }
// }
