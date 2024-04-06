use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant, SystemTime},
};

use log::info;

#[cfg(any(target_os = "linux", target = "macos"))]
use {
    libc::{mkfifo, open, read, O_RDONLY},
    std::ffi::CString,
    std::io::Error,
    std::sync::mpsc,
};

use crate::{
    events::{AFKEvent, AFKStatus, BrowserData, BrowserDataType, BrowserInformation, UserMetric},
    setting::{handle_setting_error, read_setting, Setting},
    sqlite::insert_browser_log,
};

#[cfg(any(target_os = "linux", target = "macos"))]
pub fn handle_metrics(tx: mpsc::Sender<UserMetric>) {
    let pipe_name = "/tmp/tpulse";
    match create_named_pipe(&pipe_name) {
        Ok(_) => println!("Creating named pipe successfully"),
        Err(err) => eprintln!("Error: {}", err),
    };
    loop {
        match read_from_pipe(&pipe_name) {
            Ok(data) => process_data(&data, &tx),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
fn process_data(data: &str, tx: &mpsc::Sender<UserMetric>) {
    let time_out: u64 = read_setting::<u64>(Setting::Timeout)
        .unwrap_or_else(|err| Some(handle_setting_error(Setting::Timeout, &err, 100)))
        .unwrap_or_default();

    if let Ok(parsed_data) = parse_data(&data) {
        match parsed_data.data_type {
            BrowserDataType::Tab => {
                if let Some(browser_info) = extract_browser_info(&parsed_data) {
                    info!("Browser tab: {:?}", browser_info);
                    insert_browser_log(&browser_info);
                }
            }
            BrowserDataType::VideoStatus => {
                if parsed_data.paused {
                    let parsed_data = Arc::new(parsed_data);
                    let tx = tx.clone();
                    info!("Video status: {:?}", parsed_data);

                    thread::spawn(move || {
                        let start_time = Instant::now();
                        let mut afk = false;

                        loop {
                            thread::sleep(Duration::from_secs(5));

                            if !parsed_data.paused {
                                break;
                            }

                            if start_time.elapsed().as_secs() > time_out && !afk {
                                let unix_ts = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as u64;

                                let _ = tx.send(UserMetric::AFK(AFKEvent {
                                    status: AFKStatus::OFFLINE as u8,
                                    start_time_unix: unix_ts,
                                }));
                                afk = true;
                            }
                        }
                    });
                }
            }
        }
    }
}

fn parse_data(data: &str) -> Result<BrowserData, serde_json::Error> {
    match serde_json::from_str(data) {
        Ok(parsed_data) => Ok(parsed_data),
        Err(err) => Err(err),
    }
}

fn extract_browser_info(data: &BrowserData) -> Option<BrowserInformation> {
    Some(BrowserInformation {
        start_time: data.start_time,
        title: Some(data.title.clone()),
    })
}

#[cfg(any(target_os = "linux", target = "macos"))]
fn create_named_pipe(pipe_name: &str) -> Result<(), &'static str> {
    use std::fs;

    if fs::metadata(&pipe_name).is_ok() {
        return Ok(());
    }

    let c_pipe_name = CString::new(pipe_name).expect("Failed to convert pipe name to CString");
    let result = unsafe { mkfifo(c_pipe_name.as_ptr(), 0o666) };

    if result == 0 {
        Ok(())
    } else {
        Err("Failed to create named pipe")
    }
}
#[cfg(any(target_os = "linux", target = "macos"))]
fn read_from_pipe(pipe_name: &str) -> Result<String, Error> {
    let c_pipe_name = CString::new(pipe_name).expect("Failed to convert pipe name to CString");

    let fd = unsafe { open(c_pipe_name.as_ptr(), O_RDONLY) };

    if fd == -1 {
        return Err(Error::last_os_error());
    }

    let mut buffer = String::new();
    unsafe {
        let mut byte: u8 = 0;
        loop {
            let result = read(fd, &mut byte as *mut u8 as *mut std::ffi::c_void, 1);
            if result == -1 {
                return Err(Error::last_os_error());
            } else if result == 0 {
                break;
            }
            buffer.push(byte as char);
        }
    };
    Ok(buffer)
}
#[cfg(target_os = "windows")]
use {
    std::ffi::OsStr,
    std::io::{Error, ErrorKind},
    std::os::windows::ffi::OsStrExt,
    std::ptr,
    winapi::ctypes::c_void,
    winapi::um::fileapi::ReadFile,
    winapi::um::namedpipeapi::{ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe},
    winapi::um::winbase::{
        PIPE_ACCESS_INBOUND, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
    },
    winapi::um::winnt::FILE_SHARE_READ,
};

#[cfg(target_os = "windows")]
pub fn handle_metrics() {
    let pipe_name = "\\\\.\\pipe\\tpulse";
    match create_named_pipe(&pipe_name) {
        Ok(pipe_handle) => {
            println!("Waiting for client to connect...");
            loop {
                let connected =
                    unsafe { ConnectNamedPipe(pipe_handle as *mut c_void, ptr::null_mut()) };
                if connected == 0 {
                    eprint!("Couldn't connect to named pipe")
                }
                match read_from_pipe(pipe_handle) {
                    Ok(data) => eprint!("Data from client: {}", data),
                    Err(err) => eprint!("Failed to get data from client: {}", err),
                }
                unsafe {
                    DisconnectNamedPipe(pipe_handle);
                }
            }
        }
        Err(err) => {
            eprintln!("Error creating named pipe: {}", err);
        }
    }
}

#[cfg(target_os = "windows")]
fn create_named_pipe(pipe_name: &str) -> Result<i32, Error> {
    let pipename = OsStr::new(pipe_name)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
    let pipe_handle = unsafe {
        CreateNamedPipeW(
            pipename.as_ptr(),
            PIPE_ACCESS_INBOUND | FILE_SHARE_READ,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            1,
            1024,
            1024,
            0,
            ptr::null_mut(),
        )
    };

    if pipe_handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err(Error::last_os_error());
    }
    Ok(pipe_handle as i32)
}

#[cfg(target_os = "windows")]
fn read_from_pipe(pipe_handle: i32) -> Result<String, Error> {
    const BUFFER_SIZE: usize = 1024;
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    buffer.resize(BUFFER_SIZE, 0);

    let mut bytes_read: u32 = 0;

    unsafe {
        let result = ReadFile(
            pipe_handle as *mut c_void,
            buffer.as_mut_ptr() as *mut _,
            BUFFER_SIZE as u32,
            &mut bytes_read,
            ptr::null_mut(),
        );

        if result == 0 {
            return Err(Error::last_os_error());
        }

        buffer.set_len(bytes_read as usize);
    }
    let result = String::from_utf8(buffer);
    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}
