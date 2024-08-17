// src/bindings.rs
pub mod android_native_window;
pub mod common;
pub mod games;
pub mod memory_helper;
pub mod touch_helper;
use std::error::Error;

const APP_NAME: &str = "hello world";

fn main() -> Result<(), Box<dyn Error>> {
    games::com_tencent_ig::run();

    Ok(())
}
