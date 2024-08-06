use std::ffi::{c_char, c_uchar, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct DisPlayInfo {
    pub orientation: i32,
    pub width: i32,
    pub height: i32,
}
#[link(name = "native-window", kind = "static")]

extern "C" {

    fn greeting() -> DisPlayInfo;
    fn get_display_info() -> DisPlayInfo;
    fn create_native_window(
        window_name: *const c_char,
        width: i32,
        hight: i32,
        can_screenshot: c_uchar,
    ) -> *mut c_void;
    fn destroy_native_window(win: *mut c_void);
}
pub fn safe_greeting() -> DisPlayInfo {
    return unsafe { greeting() };
}
pub fn safe_get_display_info() -> DisPlayInfo {
    unsafe {
        {
            get_display_info()
        }
    }
}
pub fn safe_create_native_window(
    window_name: &str,
    width: i32,
    hight: i32,
    can_screenshot: bool,
) -> *mut c_void {
    unsafe {
        let win_name = std::ffi::CString::new(window_name)
            .expect("window name format error,please try a right name.");

        create_native_window(win_name.as_ptr(), width, hight, {
            if can_screenshot {
                1
            } else {
                0
            }
        })
    }
}
pub fn safe_destroy_native_window(win: *mut c_void) {
    unsafe { destroy_native_window(win) }
}
