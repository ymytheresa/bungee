fn main() {
    println!("cargo:rerun-if-changed=../bungee/Bungee.h");
    println!("cargo:rerun-if-changed=wrapper.cpp");
    println!("cargo:rerun-if-changed=../src/Basic.cpp");

    // Configure C++ build
    let mut build = cc::Build::new();
    
    // Basic configuration
    build.cpp(true)
        .include("..")  // Root directory containing bungee/
        .include("../bungee")  // Bungee headers
        .include("../src")  // Source files
        .include("../submodules")  // Parent directory of KissFFT for proper include path
        .include("/opt/homebrew/Cellar/eigen/3.4.0_1/include/eigen3")  // Eigen headers
        .warnings(false)
        .flag_if_supported("-std=c++20")  // Use C++20 for std::numbers
        .define("EIGEN_NO_DEBUG", None)  // Disable Eigen assertions
        .define("NDEBUG", None)  // Disable assert() macro
        .define("EIGEN_MPL2_ONLY", None)  // Use only MPL2 licensed code
        .define("EIGEN_MAX_ALIGN_BYTES", "32");  // Fix alignment issues

    // Add wrapper file first
    build.file("wrapper.cpp");

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

    // Add KissFFT source files (now with correct paths)
    build.file("../submodules/kissfft/kiss_fft.c")
        .file("../submodules/kissfft/kiss_fftr.c");

    // Build the library
    build.compile("bungee");
} 