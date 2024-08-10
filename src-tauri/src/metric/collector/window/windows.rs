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
pub fn get_current_window_information() -> Option<Result<WindowMetric>> {
    let unix_ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    unsafe {
        let time = unix_ts.as_secs();

        let mut pid = 0;
        let hwnd = GetForegroundWindow();
        GetWindowThreadProcessId(hwnd, Option::Some(&mut pid));
        let window_title = get_window_title(hwnd).unwrap();

        let phlde: HANDLE = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).unwrap();
        let (path, name) = get_process_path_and_name(phlde);

        Some(Ok(WindowMetric {
            time,
            title: window_title,
            class: vec![name],
            exec_path: Some(path),
            category: None,
        }))
    }
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
        return Some("".to_string());
    }

    // Resize vector to actual length received from GetWindowTextW
    title_buf.truncate(len as usize);

    // Convert UTF-16 (Wide string) to UTF8 String
    let title = String::from_utf16(&title_buf).unwrap();
    Some(title)
}
