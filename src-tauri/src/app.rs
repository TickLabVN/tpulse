use crate::google::{
    __cmd__connect_google_account, __cmd__sync_google_calendar, connect_google_account,
    sync_google_calendar,
};
use tauri::{generate_context, generate_handler, App, Builder};

pub fn create_app() -> App {
    Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:tpulse.sqlite3", vec![])
                .build(),
        )
        .invoke_handler(generate_handler![
            connect_google_account,
            sync_google_calendar,
        ])
        .build(generate_context!())
        .unwrap()
}
