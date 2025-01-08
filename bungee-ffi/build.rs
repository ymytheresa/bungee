use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the headers change
    println!("cargo:rerun-if-changed=../bungee/bungee_c.h");
    println!("cargo:rerun-if-changed=../CMakeLists.txt");

    // Build the C code using CMake
    let dst = cmake::Config::new("..")
        .define("CMAKE_BUILD_TYPE", "Debug")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build();

    // Link against the Bungee library
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-search={}", dst.join("library").display());
    println!("cargo:rustc-link-search={}", dst.join("build").display());
    println!("cargo:rustc-link-lib=static=bungee_c");

    // Link against math library
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=m");
    }

    // Generate Rust bindings to bungee_c.h
    let bindings = bindgen::Builder::default()
        .header("../bungee/bungee_c.h")
        .clang_arg("-I..")  // Root include path
        .clang_arg("-DBUNGEE_DEBUG")  // Enable debug mode
        .allowlist_type("bungee_.*")
        .allowlist_function("bungee_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Output compile commands for debugging
    println!("cargo:warning=Compile commands available at: {}", 
             dst.join("compile_commands.json").display());
} 