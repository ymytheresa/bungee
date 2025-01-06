use bungee_ffi::{Stretcher, Request, Result};
use env_logger;
use log::{info, debug};
use std::f32::consts::PI;

fn generate_sine_wave(freq: f32, sample_rate: i32, duration_secs: f32) -> Vec<f32> {
    let num_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * PI * freq * t).sin();
        samples.push(sample);
    }
    
    samples
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting audio processing test");

    // Create a stretcher instance (44.1kHz stereo)
    debug!("Creating stretcher instance");
    let mut stretcher = Stretcher::new(44100, 2)?;
    info!("Stretcher created successfully");

    // Generate a test sine wave (440 Hz, 0.1 seconds)
    let sample_rate = 44100;
    debug!("Generating test sine wave");
    let input_audio = generate_sine_wave(440.0, sample_rate, 0.1);
    info!("Generated {} samples of test audio", input_audio.len());

    // Setup request for 2x speed
    let mut request = Request {
        position: 0.0,    // Start at beginning
        speed: 2.0,       // Double speed
        pitch: 1.0,       // Normal pitch
        reset: true,      // Reset state
    };

    // Preroll
    debug!("Testing preroll with request: {:?}", request);
    stretcher.preroll(&mut request)?;
    info!("Preroll successful");

    // Get max input size
    let max_frames = stretcher.max_input_frame_count();
    info!("Max input frames: {}", max_frames);

    // Process audio in chunks
    let mut position: usize = 0;
    while position < input_audio.len() {
        // Get next chunk size (convert between i32 and usize safely)
        let remaining = input_audio.len() - position;
        let chunk_size = std::cmp::min(max_frames as usize, remaining);
        debug!("Processing chunk of {} samples at position {}", chunk_size, position);

        // TODO: Once implemented, add:
        // 1. specify_grain
        // 2. analyse_grain
        // 3. synthesise_grain
        // 4. next

        position += chunk_size;
    }

    // Verify flush state
    let is_flushed = stretcher.is_flushed();
    info!("Final stretcher flushed state: {}", is_flushed);

    info!("Audio processing test completed");
    Ok(())
} 