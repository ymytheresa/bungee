use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the headers change
    println!("cargo:rerun-if-changed=../bungee/Bungee.h");
    println!("cargo:rerun-if-changed=../src/Basic.cpp");
    println!("cargo:rerun-if-changed=../CMakeLists.txt");

    // Link against the Bungee library
    println!("cargo:rustc-link-search=../build/library");
    println!("cargo:rustc-link-lib=bungee");

    // Build the C++ code using CMake
    let dst = cmake::Config::new("..")
        .define("BUNGEE_BUILD_STATIC_LIBRARIES", "ON")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    // Generate Rust bindings to Bungee.h
    let bindings = bindgen::Builder::default()
        .header("../bungee/Bungee.h")
        .clang_arg("-I..")  // Root include path
        .clang_arg("-std=c++20")
        .allowlist_type("Bungee_.*")
        .allowlist_function("Bungee_Stretcher_getFunctionTable")
        .allowlist_function("BungeePro_Stretcher_getFunctionTable")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
} 