use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    println!("cargo:rustc-link-search=native=./cxx_lib");
    //  println!("cargo:rustc-link-lib=dylib=c++_shared");
    //println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=c++_static");

    //println!("cargo:rustc-cfg=disable_exceptions");
    //  println!("cargo:rustc-link-flags=-Wl,--fno-exceptions");
}
