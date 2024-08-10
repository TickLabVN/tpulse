#[cfg(target_os = "macos")]
use {
    crate::metric::schema::WindowMetric,
    anyhow::{anyhow, Result},
    appkit_nsworkspace_bindings::{
        INSRunningApplication, INSWorkspace, NSRunningApplication, NSWorkspace, INSURL,
    },
    core_foundation::{
        base::{CFGetTypeID, ToVoid},
        mach_port::CFTypeID,
        number::{CFNumberGetType, CFNumberGetTypeID, CFNumberGetValue, CFNumberRef, CFNumberType},
        string::{CFString, CFStringGetTypeID},
    },
    core_graphics::display::*,
    objc::{
        runtime::Object,
        {msg_send, sel, sel_impl},
    },
    std::ffi::c_void,
    std::result::Result::Ok,
};

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
pub const kCFNumberSInt32Type: CFNumberType = 3;

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
pub const kCFNumberSInt64Type: CFNumberType = 4;

#[cfg(target_os = "macos")]
#[derive(Debug)]
enum DictEntryValue {
    _Number(i64),
    _String(String),
    _Unknown,
}

#[cfg(target_os = "macos")]
pub fn get_current_window_information() -> Option<WindowMetric> {
    let active_app = get_active_app();

    // Err(e) => {
    //     eprintln!("Error: {}", e);
    //     None
    // }

    match get_window_information_by_apid(active_app) {
        Ok(window_info) => Some(window_info),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

#[cfg(target_os = "macos")]
fn get_active_app() -> NSRunningApplication {
    let app_active = unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        workspace.frontmostApplication()
    };
    return app_active;
}

#[cfg(target_os = "macos")]
fn get_window_information_by_apid(app_active: NSRunningApplication) -> Result<WindowMetric> {
    const OPTIONS: CGWindowListOption =
        kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;
    let list_window_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let window_counts: isize = unsafe { CFArrayGetCount(list_window_info) };
    if window_counts <= 0 {
        return Err(anyhow!("No windows found !"));
    }
    let mut window_info = WindowMetric {
        title: "".to_string(),
        class: vec![],
        exec_path: None,
        time: 0,
        category: None,
    };
    for i in 0..window_counts {
        let dic_ref = unsafe { CFArrayGetValueAtIndex(list_window_info, i) as CFDictionaryRef };
        if dic_ref.is_null() {
            continue;
        }
        let current_active_app_pid = get_dictionary_info(dic_ref, "kCGWindowOwnerPID");
        let app_pid = get_application_pid(app_active).unwrap();
        if let DictEntryValue::_Number(pid_app) = current_active_app_pid {
            if pid_app != app_pid {
                continue;
            }
            if let DictEntryValue::_String(app_name) =
                get_dictionary_info(dic_ref, "kCGWindowOwnerName")
            {
                window_info.class = vec![app_name];
            }
            if let DictEntryValue::_String(app_title) =
                get_dictionary_info(dic_ref, "kCGWindowName")
            {
                window_info.title = app_title;
            }
            let exec_path = unsafe {
                let bundle_url = app_active.bundleURL().path();
                Some(nsstring_to_rust_string(bundle_url.0))
            };
            window_info.exec_path = exec_path;
            return Ok(window_info);
        }
    }
    unsafe { CFRelease(list_window_info as CFTypeRef) }
    Err(anyhow!("There's an error"))
}

#[cfg(target_os = "macos")]
fn get_application_pid(app_active: NSRunningApplication) -> Result<i64> {
    let app_pid = unsafe { app_active.processIdentifier() as i64 };
    return Ok(app_pid);
}

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
fn get_dictionary_info(dict: CFDictionaryRef, key: &str) -> DictEntryValue {
    let cf_key: CFString = key.into();
    let mut value: *const c_void = std::ptr::null();
    if unsafe { CFDictionaryGetValueIfPresent(dict, cf_key.to_void(), &mut value) } != 0 {
        let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
        if type_id == unsafe { CFNumberGetTypeID() } {
            let value = value as CFNumberRef;
            match unsafe { CFNumberGetType(value) } {
                kCFNumberSInt64Type => {
                    let mut value_i64 = 0_i64;
                    let out_value: *mut i64 = &mut value_i64;
                    let converted =
                        unsafe { CFNumberGetValue(value, kCFNumberSInt64Type, out_value.cast()) };
                    if converted {
                        return DictEntryValue::_Number(value_i64);
                    }
                }
                kCFNumberSInt32Type => {
                    let mut value_i32 = 0_i32;
                    let out_value: *mut i32 = &mut value_i32;
                    let converted =
                        unsafe { CFNumberGetValue(value, kCFNumberSInt32Type, out_value.cast()) };
                    if converted {
                        return DictEntryValue::_Number(value_i32 as i64);
                    }
                }
                _ => {
                    eprintln!("Unexpected Type: {}", type_id);
                }
            }
        } else if type_id == unsafe { CFStringGetTypeID() } {
            let strr = nsstring_to_rust_string(value as *mut Object);
            return DictEntryValue::_String(strr);
        } else {
            eprintln!("Unexpected Type: {}", type_id);
        }
    }
    return DictEntryValue::_Unknown;
}

#[cfg(target_os = "macos")]
pub fn nsstring_to_rust_string(nsstring: *mut Object) -> String {
    unsafe {
        let cstr: *const i8 = msg_send![nsstring, UTF8String];
        if !cstr.is_null() {
            std::ffi::CStr::from_ptr(cstr)
                .to_string_lossy()
                .into_owned()
        } else {
            "".into()
        }
    }
}
