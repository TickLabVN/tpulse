use tauri::{App, Builder};
use tauri_plugin_log::{Target, TargetKind};

pub fn create_app() -> App {
    Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:tpulse.sqlite3", vec![])
                .build(),
        )
        .build(tauri::generate_context!())
        .unwrap()
}
