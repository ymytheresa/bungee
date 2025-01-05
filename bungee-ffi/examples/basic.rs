use bungee_ffi::{Stretcher, Request};
use env_logger::Env;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .init();

    // Create a new stretcher instance (44.1kHz, stereo)
    let mut stretcher = Stretcher::new(44100, 2)
        .expect("Failed to create stretcher");

    // Create a request with some example parameters
    let mut request = Request {
        position: 0.0,  // Start from beginning
        speed: 0.5,    // Half speed
        pitch: 1.0,    // No pitch change
        reset: true,   // Start fresh
    };

    // Test preroll
    stretcher.preroll(&mut request)
        .expect("Failed to preroll");

    // Get max input frame count
    let max_frames = stretcher.max_input_frame_count();
    println!("Max input frames: {}", max_frames);

    // Check if flushed
    let flushed = stretcher.is_flushed();
    println!("Stretcher flushed: {}", flushed);
} 