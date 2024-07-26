// src/bindings.rs
pub mod android_native_window;
use android_native_window::{safe_create_native_window, safe_destroy_native_window, safe_greeting};

fn main() {
    let info = safe_greeting();
    println!("{:?}", info);

    let win = safe_create_native_window("test", 1080, 2400, true);
    println!("this win's addr is {:?}", win);
    safe_destroy_native_window(win);
}
