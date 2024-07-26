use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    //config APP_STL = c++_static and static link libnative-window.a for local module android_native_window 
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-search=native=/home/jinx/android-ndk-r23c/sources/cxx-stl/llvm-libc++/libs/arm64-v8a/");
    println!("cargo:rustc-link-lib=static=c++_static");
    println!("cargo:rustc-link-lib=static=c++abi");
}
