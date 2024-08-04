mod utils;
use crate::metric::schema::Activity;
use log::{error, info};
use std::sync::mpsc;
use utils::{convert_to_user_metric, create_named_pipe, read_from_pipe};

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn watch_browser(tx: mpsc::Sender<Activity>) {
    info!("Watching browser");
    let pipe_name = "/tmp/tpulse";
    if let Err(err) = create_named_pipe(&pipe_name) {
        error!("Error: {}", err);
    }

    loop {
        match read_from_pipe(&pipe_name)
            .map_err(|e| e.to_string())
            .and_then(|mut v| convert_to_user_metric(&mut v).map_err(|e| e.to_string()))
        {
            Ok(metric) => {
                for m in metric {
                    if let Err(e) = tx.send(m) {
                        error!("Failed to send browser metric: {}", e);
                    }
                }
            }
            Err(err) => error!("Error: {}", err),
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
pub fn watch_browser(tx: mpsc::Sender<Activity>) {
    let pipe_name = "\\\\.\\pipe\\tpulse";
    match create_named_pipe(&pipe_name) {
        Ok(pipe_handle) => {
            info!("Watching browser...");
            loop {
                let connected =
                    unsafe { ConnectNamedPipe(pipe_handle as *mut c_void, ptr::null_mut()) };
                if connected == 0 {
                    eprintln!("Couldn't connect to named pipe");
                }
                match read_from_pipe(pipe_handle)
                    .map_err(|e| e.to_string())
                    .and_then(|mut v| convert_to_user_metric(&mut v).map_err(|e| e.to_string()))
                {
                    Ok(metric) => {
                        for m in metric {
                            if let Err(e) = tx.send(m) {
                                error!("Failed to send browser metric: {}", e);
                            }
                        }
                    }
                    Err(err) => error!("Error: {}", err),
                }
                unsafe {
                    DisconnectNamedPipe(pipe_handle as *mut c_void);
                }
            }
        }
        Err(err) => {
            error!("Error creating named pipe: {}", err);
        }
    }
}
