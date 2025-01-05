fn main() {
    // Build Bungee C++ library using CMake
    let dst = cmake::build("../");

    // Tell cargo to link the bungee library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=bungee");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/bungee.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.cpp");

    // Generate Rust bindings using cxx
    let mut build = cxx_build::bridge("src/bungee.rs");
    
    // Get output directory for generated files
    let out_dir = std::env::var("OUT_DIR").unwrap();
    
    // Add include paths in correct order
    build.include(&format!("{}/cxxbridge/include", out_dir)); // CXX runtime headers first
    build.include(&format!("{}/cxxbridge/crate", out_dir));  // Generated headers second
    build.include(".");  // Local headers third
    build.include(".."); // Parent directory for bungee headers
    build.include("../bungee"); // Bungee headers last
    
    // Set C++ standard and flags
    build.flag_if_supported("-std=c++17");
    
    // M1-specific configurations
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        build.flag("-arch")
             .flag("arm64")
             .flag("-mcpu=apple-m1")
             .flag("-mmacosx-version-min=11.0");
    }
    
    // Add source files
    build.file("wrapper.cpp");
    
    // Print include paths for debugging
    println!("\nCXX include path:");
    println!("  {}/cxxbridge/include", out_dir);
    println!("  {}/cxxbridge/crate", out_dir);
    
    // Compile
    build.compile("bungee-cxx");
    
    // Link C++ standard library
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
    }
    
    // Print CXX bridge information
    println!("cargo:CXXBRIDGE_PREFIX=bungee-rs");
    println!("cargo:CXXBRIDGE_DIR0={}/cxxbridge/include", out_dir);
    println!("cargo:CXXBRIDGE_DIR1={}/cxxbridge/crate", out_dir);
} 