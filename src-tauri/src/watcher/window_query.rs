use std::time::SystemTime;
use crate::dtos::WindowInformation;

#[cfg(target_os = "linux")]
use {
    anyhow::{anyhow, Ok, Result},
    regex::Regex,
    std::fs,
    std::process::Command,
};

#[cfg(target_os = "linux")]
pub fn get_current_window_information() -> Result<WindowInformation> {

    let window_raw_id = get_window_id().unwrap();
    let window_info = get_window_information_by_id(window_raw_id)?;
    Ok(window_info)
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
fn get_window_information_by_id(window_id: i64) -> Result<WindowInformation> {
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
        return Err(anyhow!("Get window information error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&window_raw_infor.stdout);
    let unix_ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let mut window_info = WindowInformation {
        time: unix_ts.as_secs(),
        title: None,
        class: None,
        exec_path: None,
    };

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "WM_NAME" => window_info.title = Some(parts[1].trim_matches('"').to_string()),
            "WM_CLASS" => {
                let class = parts[1].split(',').map(|s| s.trim().trim_matches('"'));
                window_info.class = Some(class.map(|s| s.to_string()).collect());
            }
            "_NET_WM_PID" => {
                let pid = parts[1].trim().parse::<i32>().unwrap();
                if let std::result::Result::Ok(exec_path) =
                    fs::read_link(format!("/proc/{}/exe", pid))
                {
                    let path_str = exec_path.as_path().display().to_string();
                    window_info.exec_path = Some(path_str);
                }
            }
            "_NET_WM_NAME" => {
                if window_info.title.is_none() {
                    window_info.title = Some(parts[1].trim_matches('"').to_string());
                }
            }
            _ => {}
        }
    }

    Ok(window_info)
}

#[cfg(target_os = "windows")]
use {
    anyhow::Result,
    windows::core::PWSTR,
    windows::Win32::Foundation::{HANDLE, HWND, MAX_PATH},
    windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT,
        PROCESS_QUERY_LIMITED_INFORMATION,
    },
    windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    },
};

#[cfg(target_os = "windows")]
pub fn get_current_window_information() -> Result<WindowInformation> {
    let mut window_info = WindowInformation {
        time: 0,
        title: None,
        class: None,
        exec_path: None,
    };
    unsafe {
        let mut pid = 0;
        let hwnd = GetForegroundWindow();
        GetWindowThreadProcessId(hwnd, Option::Some(&mut pid));
        let window_title = get_window_title(hwnd).unwrap();

        let phlde: HANDLE = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).unwrap();
        let (path, name) = get_process_path_and_name(phlde);

        let unix_ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        window_info.time = unix_ts.as_millis();
        window_info.title = Some(window_title);
        window_info.exec_path = Some(path);
        window_info.class = Some(vec![name]);
    }
    Ok(window_info)
}
#[cfg(target_os = "windows")]
fn get_process_path_and_name(phlde: HANDLE) -> (String, String) {
    // Allocate a buffer to store the path on stack
    let mut buf = [0u16; MAX_PATH as usize];
    let lpexename = PWSTR::from_raw(buf.as_mut_ptr());
    let mut dw_size = MAX_PATH as u32;
    unsafe {
        QueryFullProcessImageNameW(
            phlde,
            PROCESS_NAME_FORMAT::default(),
            lpexename,
            &mut dw_size,
        )
        .unwrap();

        let path = lpexename.to_string().unwrap();

        let separator = if cfg!(windows) { '\\' } else { '/' };

        let name = if let Some(index) = path.rfind(separator) {
            path[(index + 1)..].to_string()
        } else {
            String::new()
        };

        (path, name)
    }
}

#[cfg(target_os = "windows")]
fn get_window_title(hwnd: HWND) -> Option<String> {
    let mut buf_size = unsafe { GetWindowTextLengthW(hwnd) };
    buf_size += 1; // for '\0' terminator
    let mut title_buf: Vec<u16> = vec![0; buf_size as usize];

    let len = unsafe { GetWindowTextW(hwnd, &mut title_buf) };
    if len == 0 {
        return None;
    }

    // Resize vector to actual length received from GetWindowTextW
    title_buf.truncate(len as usize);

    // Convert UTF-16 (Wide string) to UTF8 String
    let title = String::from_utf16(&title_buf).unwrap();
    Some(title)
}

#[cfg(target_os = "macos")]
pub fn get_current_window_information() -> Result<WindowInformation> {
    unimplemented!("MacOS hasn't been supported yet.")
}
