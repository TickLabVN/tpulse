mod utils;
use utils::{create_named_pipe, read_from_pipe};

#[cfg(any(target_os = "linux", target = "macos"))]
pub fn watch_browser() {
    let pipe_name = "/tmp/tpulse";
    match create_named_pipe(&pipe_name) {
        Ok(_) => println!("Creating named pipe successfully"),
        Err(err) => eprintln!("Error: {}", err),
    };
    loop {
        match read_from_pipe(&pipe_name) {
            Ok(data) => println!("Data read from the pipe: {}", data),
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
pub fn watch_browser() {
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
                    DisconnectNamedPipe(pipe_handle as *mut c_void);
                }
            }
        }
        Err(err) => {
            eprintln!("Error creating named pipe: {}", err);
        }
    }
}
