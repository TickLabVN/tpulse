use std::ffi::CString;
use log::info;
use x11::xlib::{XCloseDisplay, XDefaultScreen, XOpenDisplay, XQueryPointer, XRootWindow, XWindowAttributes, XGetWindowAttributes, XFetchName, XClassHint, XGetClassHint};
pub struct CurrentWindow {
    title: String,
    class: String,
}

impl CurrentWindow {
    pub fn new() -> Self {
        let display = unsafe { XOpenDisplay(std::ptr::null()) };
        if display.is_null() {
            panic!("Failed to open X display");
        }

        let screen = unsafe { XDefaultScreen(display) };
        let root_window = unsafe { XRootWindow(display, screen) };

        // Get the current window
        let mut root_return = 0;
        let mut child_return = 0;
        let mut root_x_return = 0;
        let mut root_y_return = 0;
        let mut win_x_return = 0;
        let mut win_y_return = 0;
        let mut mask_return = 0;
        let success = unsafe { XQueryPointer(
            display,
            root_window,
            &mut root_return,
            &mut child_return,
            &mut root_x_return,
            &mut root_y_return,
            &mut win_x_return,
            &mut win_y_return,
            &mut mask_return,
        ) };

        if success == 0 {
            panic!("Failed to query pointer");
        }

        info!("root_return: {}", root_return);

        // Get window attributes
        let mut window_attributes = XWindowAttributes {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            border_width: 0,
            depth: 0,
            visual: std::ptr::null_mut(),
            root: 0,
            class: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            colormap: 0,
            map_installed: 0,
            map_state: 0,
            all_event_masks: 0,
            your_event_mask: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            screen: std::ptr::null_mut(),
        };
        info!("window_attributes: {:?}", window_attributes);
        unsafe { XGetWindowAttributes(display, root_window, &mut window_attributes) };

        // Get window title
        let mut window_title: *mut i8 = std::ptr::null_mut();
        unsafe { XFetchName(display, root_window, &mut window_title) };
        if !window_title.is_null() {
            let title = unsafe { CString::from_raw(window_title).into_string().unwrap() };
            println!("Window Title: {}", title);
        }

        // Get window class
        let mut window_class_hint = XClassHint {
            res_name: std::ptr::null_mut(),
            res_class: std::ptr::null_mut(),
        };
        info!("window_class_hint: {:?}", window_class_hint  );
        let mut class_return = std::ptr::null_mut::<i8>();
        unsafe { XGetClassHint(display, root_window, &mut window_class_hint) };
        if !window_class_hint.res_class.is_null() {
            class_return = window_class_hint.res_class;
        }
        if !class_return.is_null() {
            let class = unsafe { CString::from_raw(class_return).into_string().unwrap() };
            println!("Window Class: {}", class);
        }

        // Close the display connection
        unsafe { XCloseDisplay(display) };
        CurrentWindow {
            title: "QEQWE".to_owned(),
            class: "QWEQWE".to_owned(),
        }
    }
}
