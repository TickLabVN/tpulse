#[cfg(target_os = "linux")]
use {
    libc::{mkfifo, open, read, O_RDONLY},
    std::ffi::CString,
    std::io::Error,
};
#[cfg(target_os = "linux")]
pub fn handle_metrics() {
    let pipe_name = "/tmp/tpulse-test13";
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
#[cfg(target_os = "linux")]
fn create_named_pipe(pipe_name: &str) -> Result<(), &'static str> {
    let c_pipe_name = CString::new(pipe_name).expect("Failed to convert pipe name to CString");
    let result = unsafe { mkfifo(c_pipe_name.as_ptr(), 0o666) };

    if result == 0 {
        Ok(())
    } else {
        Err("Failed to create named pipe")
    }
}
#[cfg(target_os = "linux")]
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
    std::ffi::CString,
    std::io::Error,
    winapi::um::fileapi::{FILE_FLAG_OVERLAPPED, OPEN_EXISTING},
    winapi::um::namedpipeapi::{ConnectNamedPipe, CreateNamedPipe, DisconnectNamedPipe},
    winapi::um::winbase::{PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE},
    winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE},
};

#[cfg(target_os = "windows")]
pub fn handle_metrics() {
    let mut pipe_name = "\\\\.\\pipe\\tpulse";
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
fn create_named_pipe(pipe_name: &str) {
    let pipe_name = CString::new(pipe_name).expect("Failed to convert pipe name to CString");
    let pipe_handle = unsafe {
        CreateNamedPipe(
            pipe_name.as_ptr(),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE,
            1,
            0,
            0,
            0,
            ptr::null_mut(),
        )
    };

    if pipe_handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        panic!("Failed to create named pipe");
    }
    pipe_handle();
}

#[cfg(target_os = "windows")]
fn read_from_pipe(pipe_name: &str) -> io::Result<String> {
    let pipe_name = CString::new(pipe_name).expect("Failed to convert pipe name to CString");

    let pipe_handle = unsafe {
        CreateFileW(
            pipe_name.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED,
            ptr::null_mut(),
        )
    };

    if pipe_handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }
    let connected = unsafe { ConnectNamedPipe(pipe_handle, ptr::null_mut()) };
    if connected == 0 {
        return Err(io::Error::last_os_error());
    }

    let mut buffer = String::new();
    let mut byte: u8 = 0;
    loop {
        let result = unsafe {
            ReadFile(
                pipe_handle,
                &mut byte as *mut u8 as *mut _,
                1,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if result == 0 {
            break;
        }
        buffer.push(byte);
    }
    let result = String::from_utf8(buffer);
    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}
