mod bungee;

fn main() {
    println!("[Rust] Starting Bungee wrapper test");

    // Create a new stretcher instance (44.1kHz, stereo)
    println!("[Rust] Creating stretcher with sample_rate=44100, channels=2");
    let stretcher = bungee::ffi::new_stretcher(44100, 2);

    // Create a request with some example parameters
    let mut request = bungee::ffi::Request {
        pitch: 1.0,  // No pitch change
        speed: 0.5,  // Half speed
        position: 0.0,  // Start from beginning
    };
    println!("[Rust] Created initial request: {:?}", request);

    // Test preroll
    println!("[Rust] Testing preroll...");
    stretcher.preroll(&mut request);
    println!("[Rust] After preroll, request is: {:?}", request);

    // Test specify_grain
    println!("[Rust] Testing specify_grain...");
    let input_chunk = stretcher.specify_grain(&request);
    println!("[Rust] Got input chunk: offset={}, length={}", input_chunk.offset, input_chunk.length);

    // Create some dummy audio data for testing
    println!("[Rust] Creating dummy audio data for testing");
    let dummy_data: Vec<f32> = vec![0.0; input_chunk.length as usize];
    
    // Test analyse_grain
    println!("[Rust] Testing analyse_grain...");
    stretcher.analyse_grain(&dummy_data, 1);
    println!("[Rust] Analysis complete");

    // Test synthesise_grain
    println!("[Rust] Testing synthesise_grain...");
    let output_chunk = stretcher.synthesise_grain();
    println!("[Rust] Got output chunk with {} samples", output_chunk.data.len());
    println!("[Rust] Start request: {:?}", output_chunk.start_request);
    println!("[Rust] End request: {:?}", output_chunk.end_request);

    // Test next
    println!("[Rust] Testing next...");
    stretcher.next(&mut request);
    println!("[Rust] After next, request is: {:?}", request);

    println!("[Rust] Test complete!");
}
