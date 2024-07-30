use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {

    println!("cargo:rustc-link-search=native=./cxx_lib");

}
