fn main() {
    println!("cargo:rerun-if-changed=../bungee/Bungee.h");
    println!("cargo:rerun-if-changed=../src/Basic.cpp");

    // Configure C++ build
    let mut build = cc::Build::new();
    
    // Basic configuration
    build.cpp(true)
        .include("..")  // Root directory containing bungee/
        .include("/opt/homebrew/include")  // Eigen headers
        .warnings(false)
        .flag_if_supported("-std=c++17");  // Use C++17 for nested namespaces

    // Add all source files
    let source_files = [
        "Basic.cpp",
        "Assert.cpp",
        "Fourier.cpp",
        "Grain.cpp",
        "Grains.cpp",
        "Input.cpp",
        "Output.cpp",
        "Partials.cpp",
        "Stretch.cpp",
        "Synthesis.cpp",
        "Timing.cpp",
        "Window.cpp",
    ];

    for file in source_files.iter() {
        println!("cargo:rerun-if-changed=../src/{}", file);
        build.file(format!("../src/{}", file));
    }

    // Build the library
    build.compile("bungee");
} 