#[cfg(target_os = "macos")]
use {
    crate::metric::schema::WindowMetric,
    appkit_nsworkspace_bindings::{
        INSRunningApplication, INSWorkspace, NSRunningApplication, NSWorkspace, INSURL,
    },
    core_foundation::{
        base::ToVoid,
        number::{CFNumberGetValue, CFNumberRef, CFNumberType},
        string::CFString,
    },
    core_graphics::display::*,
    objc::{
        runtime::Object,
        {msg_send, sel, sel_impl},
    },
    std::ffi::c_void,
    std::time::SystemTime,
};

#[cfg(target_os = "macos")]
const WHITE_LIST_WINDOWS: CGWindowListOption =
    kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
const kCGWindowOwnerPID: &str = "kCGWindowOwnerPID";

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
const kCGWindowOwnerName: &str = "kCGWindowOwnerName";

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
const kCGWindowName: &str = "kCGWindowName";

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
pub const kCFNumberInt64Type: CFNumberType = 4;

#[cfg(target_os = "macos")]
pub fn get_current_window_information() -> Option<WindowMetric> {
    let unix_ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let active_app = get_active_app();

    let result = get_dictionary_ref(active_app);

    match result {
        Some(dict_ref) => {
            let mut window_info = WindowMetric::default();
            if let Some(title) = get_dict_title(dict_ref) {
                window_info.title = title;
            }

            if let Some(class) = get_dict_class(dict_ref) {
                window_info.class = vec![class];
            }

            let exec_path = unsafe {
                let bundle_url = active_app.bundleURL().path();
                Some(nsstring_to_rust_string(bundle_url.0))
            };

            window_info.exec_path = exec_path;

            window_info.time = unix_ts.as_secs();

            return Some(window_info);
        }

        None => {
            eprintln!("Failed to get window information");
            return None;
        }
    }
}

/// Run `ps -p <pid>` to get the PID of the active app
#[cfg(target_os = "macos")]
fn get_active_app() -> NSRunningApplication {
    let app_active = unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        workspace.frontmostApplication()
    };
    return app_active;
}

#[cfg(target_os = "macos")]
fn get_dictionary_ref(app_active: NSRunningApplication) -> Option<CFDictionaryRef> {
    let window_info_list =
        unsafe { CGWindowListCopyWindowInfo(WHITE_LIST_WINDOWS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_info_list) };

    // FIXME: Although we have gotten the current active app, why do we need to check window_counts?
    if window_info_list.is_null() {
        return None;
    }

    let app_pid = get_application_pid(app_active);

    for i in 0..count {
        let dic_ref =
            unsafe { CFArrayGetValueAtIndex(window_info_list, i as isize) as CFDictionaryRef };

        if dic_ref.is_null() {
            continue;
        }

        let dict_pid = get_dict_pid(dic_ref)?;

        if dict_pid == app_pid {
            return Some(dic_ref);
        }
    }

    return None;
}

#[cfg(target_os = "macos")]
fn get_application_pid(app_active: NSRunningApplication) -> i64 {
    let app_pid = unsafe { app_active.processIdentifier() as i64 };
    return app_pid;
}

#[cfg(target_os = "macos")]
fn get_dict_pid(dic_ref: CFDictionaryRef) -> Option<i64> {
    let mut value = std::ptr::null();
    let flag = unsafe {
        CFDictionaryGetValueIfPresent(
            dic_ref,
            CFString::from(kCGWindowOwnerPID).to_void(),
            &mut value as *mut *const c_void,
        )
    };

    if flag == 0 {
        return None;
    }

    let mut pid = 0_i64;

    let cf_ref = value as CFNumberRef;
    let out_value: *mut i64 = &mut pid;
    let converted = unsafe { CFNumberGetValue(cf_ref, kCFNumberInt64Type, out_value.cast()) };

    if converted {
        return Some(pid);
    }

    return None;
}

#[cfg(target_os = "macos")]
fn get_dict_title(dic_ref: CFDictionaryRef) -> Option<String> {
    return get_dict_title_or_class(dic_ref, kCGWindowName);
}

#[cfg(target_os = "macos")]
fn get_dict_class(dic_ref: CFDictionaryRef) -> Option<String> {
    return get_dict_title_or_class(dic_ref, kCGWindowOwnerName);
}

#[cfg(target_os = "macos")]
fn get_dict_title_or_class(dic_ref: CFDictionaryRef, key: &str) -> Option<String> {
    let mut value = std::ptr::null();
    let flag = unsafe {
        CFDictionaryGetValueIfPresent(
            dic_ref,
            CFString::from(key).to_void(),
            &mut value as *mut *const c_void,
        )
    };

    if flag == 0 {
        return None;
    }

    let title = nsstring_to_rust_string(value as *mut Object);

    return Some(title);
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
