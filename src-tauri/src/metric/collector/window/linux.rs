use crate::metric::schema::WindowMetric;
use std::time::SystemTime;

#[cfg(target_os = "linux")]
use {
    anyhow::{anyhow, Ok, Result},
    regex::Regex,
    std::fs,
    std::process::Command,
};

#[cfg(target_os = "linux")]
pub fn get_current_window_information() -> Option<WindowMetric> {
    let window_raw_id = get_window_id().unwrap();
    if window_raw_id == 0 {
        // No open window found
        return None;
    }
    
    get_window_information_by_id(window_raw_id)
}

#[cfg(target_os = "linux")]
fn get_window_id() -> Result<i64> {
    let bin = "xprop";
    let args = ["-root", "_NET_ACTIVE_WINDOW"];
    let window_id_regex = Regex::new(r"0x[a-fA-F0-9]+").unwrap();
    let window_raw_id = Command::new(bin)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    if !window_raw_id.status.success() {
        let stderr = String::from_utf8_lossy(&window_raw_id.stderr);
        return Err(anyhow!("Get window ID error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&window_raw_id.stdout);
    if let Some(captured) = window_id_regex.find(&stdout) {
        let window_id_hex = captured.as_str().trim_start_matches("0x");
        let window_id = i64::from_str_radix(window_id_hex, 16).unwrap();
        return Ok(window_id);
    }
    Err(anyhow!("No window ID found in the input string."))
}

#[cfg(target_os = "linux")]
fn get_window_information_by_id(window_id: i64) -> Option<WindowMetric> {
    let bin = "xprop";
    let window_raw_infor = Command::new(bin)
        .env("LC_ALL", "C.utf8")
        .arg("-id")
        .arg(window_id.to_string())
        .arg("-notype")
        .arg("WM_NAME")
        .arg("WM_CLASS")
        .arg("_NET_WM_NAME")
        .arg("_NET_WM_PID")
        .output()
        .expect("Failed to execute command");

    if !window_raw_infor.status.success() {
        let stderr = String::from_utf8_lossy(&window_raw_infor.stderr);
        log::error!("Get window information error: {}", stderr);
        return None;
    }

    let stdout = String::from_utf8_lossy(&window_raw_infor.stdout);
    let unix_ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let mut title: Option<String> = None;
    let time = unix_ts.as_secs();
    let mut class: Option<Vec<String>> = None;
    let mut exec_path: Option<String> = None;

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "WM_NAME" => title = Some(parts[1].trim_matches('"').to_string()),
            "WM_CLASS" => {
                let wm_class = parts[1].split(',').map(|s| s.trim().trim_matches('"'));
                class = Some(wm_class.map(|s| s.to_string()).collect());
            }
            "_NET_WM_PID" => {
                let pid = parts[1].trim().parse::<i32>().unwrap();
                if let std::result::Result::Ok(path) = fs::read_link(format!("/proc/{}/exe", pid)) {
                    let path_str = path.as_path().display().to_string();
                    exec_path = Some(path_str);
                }
            }
            "_NET_WM_NAME" => {
                if title.is_none() {
                    title = Some(parts[1].trim_matches('"').to_string());
                }
            }
            _ => {}
        }
    }

    let window = WindowMetric {
        time,
        title: title.unwrap(),
        class: class.unwrap(),
        exec_path,
        category: None,
    };

    Some(window)
}
