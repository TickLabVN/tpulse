mod utils;
use crate::metrics::UserMetric;
use std::sync::mpsc;

use utils::{convert_to_user_metric, create_named_pipe, read_from_pipe};
#[cfg(any(target_os = "linux", target = "macos"))]

pub fn watch_browser(tx: mpsc::Sender<UserMetric>) {
    use log::error;
    let pipe_name = "/tmp/tpulse";
    if let Err(err) = create_named_pipe(&pipe_name)  {
        error!("Error: {}", err);
    }
    loop {
        match read_from_pipe(&pipe_name)
            .map_err(|e| e.to_string())
            .and_then(|v| convert_to_user_metric(v).map_err(|e| e.to_string()))
        {
            Ok(metric) => {
                if let Err(_) = tx.send(metric) {
                    eprintln!("Failed to send browser metric");
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

#[cfg(target_os = "windows")]
use {
    std::ptr,
    winapi::ctypes::c_void,
    winapi::um::namedpipeapi::{ConnectNamedPipe, DisconnectNamedPipe},
};

#[cfg(target_os = "windows")]
pub fn watch_browser(tx: mpsc::Sender<UserMetric>) {
    let pipe_name = "\\\\.\\pipe\\tpulse";
    match create_named_pipe(&pipe_name) {
        Ok(pipe_handle) => {
            println!("Waiting for client to connect...");
            loop {
                let connected =
                    unsafe { ConnectNamedPipe(pipe_handle as *mut c_void, ptr::null_mut()) };
                if connected == 0 {
                    eprintln!("Couldn't connect to named pipe");
                }
                match read_from_pipe(&pipe_name)
                    .map_err(|e| e.to_string())
                    .and_then(|v| convert_to_user_metric(v).map_err(|e| e.to_string()))
                {
                    Ok(metric) => {
                        if let Err(_) = tx.send(metric) {
                            eprintln!("Failed to send browser metric");
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
                unsafe {
                    DisconnectNamedPipe(pipe_handle as *mut c_void);
                }
            }
        }
        Err(err) => {
            eprintln!("Error creating named pipe: {}", err);
        }
    }
}
