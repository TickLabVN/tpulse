#[tauri::command]
pub fn get_data_directory() -> String {
    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let data_dir = home_dir.join(".ticklabvn.tpulse");
        data_dir.to_string_lossy().into_owned()
    }

    #[cfg(target_os = "windows")]
    {
        let app_data_dir = dirs::data_local_dir().expect("Failed to get local app data directory");
        let data_dir = app_data_dir.join(".ticklabvn.tpulse"); // Change "YourAppName" to your actual app name
        data_dir.to_string_lossy().into_owned()
    }

    #[cfg(target_os = "macos")]
    {
        let app_support_dir =
            dirs::data_local_dir().expect("Failed to get local app support directory");
        let data_dir = app_support_dir.join(".ticklabvn.tpulse"); // Change "YourAppName" to your actual app name
        data_dir.to_string_lossy().into_owned()
    }
}
