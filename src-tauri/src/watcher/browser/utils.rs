// The browser tpulse extension sends read data to a named pipe
// Our app reads data from this named pipe to retrieve data from browser tabs

#[cfg(any(target_os = "linux", target = "macos"))]
use {
    libc::{mkfifo, open, read, O_RDONLY},
    std::ffi::CString,
    std::io::Error,
};

#[cfg(any(target_os = "linux", target = "macos"))]
pub fn create_named_pipe(pipe_name: &str) -> Result<(), &'static str> {
    let c_pipe_name = CString::new(pipe_name).expect("Failed to convert pipe name to CString");
    let result = unsafe { mkfifo(c_pipe_name.as_ptr(), 0o666) };

    if result == 0 {
        Ok(())
    } else {
        Err("Failed to create named pipe")
    }
}

#[cfg(any(target_os = "linux", target = "macos"))]
pub fn read_from_pipe(pipe_name: &str) -> Result<String, Error> {
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
    winapi::um::namedpipeapi::CreateNamedPipeW,
    winapi::um::winbase::{
        PIPE_ACCESS_INBOUND, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
    },
    winapi::um::winnt::FILE_SHARE_READ,
};

#[cfg(target_os = "windows")]
pub fn create_named_pipe(pipe_name: &str) -> Result<i32, Error> {
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
pub fn read_from_pipe(pipe_handle: i32) -> Result<String, Error> {
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
