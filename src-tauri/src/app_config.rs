use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    settings: HashMap<SettingName, Option<String>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut settings = HashMap::new();
        settings.insert(SettingName::PollTime, Some("500".to_string()));
        settings.insert(SettingName::Timeout, Some("1000".to_string()));

        AppConfig { settings }
    }
}

// add new settings here
#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum SettingName {
    PollTime,
    Timeout,
    GoogleRefreshToken,
    PkceVerifier,
    RedirectPort,
    GoogleAccessToken,
}

impl AppConfig {
    fn get_setting(&self, setting: &SettingName) -> Option<&Option<String>> {
        self.settings.get(setting)
    }

    fn set_setting<T>(&mut self, setting: SettingName, new_value: Option<T>)
    where
        T: Into<String>,
    {
        self.settings.insert(setting, new_value.map(Into::into));
    }
}

const CONFIG_FILE_PATH: &str = "setting.json";

fn read_all_settings_from_file() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(CONFIG_FILE_PATH)?;
    let config: AppConfig = serde_json::from_str(&file_contents)?;
    Ok(config)
}

fn write_all_settings_to_file(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let json_config = serde_json::to_string_pretty(&config)?;
    fs::write(CONFIG_FILE_PATH, json_config)?;
    Ok(())
}

pub fn read_setting<T>(setting_name: SettingName) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    let settings = read_all_settings_from_file().unwrap_or_else(|err| {
        handle_setting_error(SettingName::Timeout, &err, AppConfig::default())
    });

    if let Some(setting_value) = settings.get_setting(&setting_name).cloned().flatten() {
        let parsed_value: T = serde_json::from_str(&setting_value)?;
        Ok(Some(parsed_value))
    } else {
        Ok(None)
    }
}

pub fn write_setting(
    setting_name: SettingName,
    new_value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = read_all_settings_from_file().unwrap_or_else(|err| {
        handle_setting_error(SettingName::Timeout, &err, AppConfig::default())
    });
    settings.set_setting(setting_name, Some(new_value));

    write_all_settings_to_file(&settings)?;

    Ok(())
}

pub fn remove_setting(setting_name: SettingName) -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = read_all_settings_from_file()?;
    settings.settings.remove(&setting_name);
    write_all_settings_to_file(&settings)?;
    Ok(())
}

pub fn handle_setting_error<T>(setting: SettingName, err: &Box<dyn Error>, default_value: T) -> T
where
    T: Default,
{
    eprintln!("Error reading {:?} setting: {}", setting, err);
    default_value
}
