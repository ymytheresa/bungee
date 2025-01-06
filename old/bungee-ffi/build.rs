use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=../bungee/bungee_c.h");
    println!("cargo:rerun-if-changed=../bungee/bungee_c.cpp");
    
    // Build bungee library
    let dst = cmake::Config::new("..")
        .define("BUNGEE_BUILD_STATIC_LIBRARIES", "ON")
        .build();

    // Link against bungee library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=bungee");
    
    // Link against C++ standard library on macOS
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }
} 