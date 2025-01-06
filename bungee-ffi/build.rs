use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=include/bungee_ffi.h");
    println!("cargo:rerun-if-changed=src/bungee_ffi.cpp");

    // Build the C++ code
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .file("src/bungee_ffi.cpp")
        .include("include")
        .include("../bungee")  // Main Bungee header location
        .flag_if_supported("-std=c++17")
        .compile("bungee_ffi");

    // Generate Rust bindings
    let bindings = bindgen::Builder::default()
        .header("include/bungee_ffi.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
} 