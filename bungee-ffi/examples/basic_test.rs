use bungee_ffi::{Stretcher, Request, Result};
use env_logger;
use log::{info, debug};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting basic Bungee test");

    // Create a stretcher instance (44.1kHz stereo)
    debug!("Creating stretcher instance");
    let mut stretcher = Stretcher::new(44100, 2)?;
    info!("Stretcher created successfully");

    // Try basic operations
    let mut request = Request {
        position: 0.0,    // Start at beginning
        speed: 1.0,       // Normal speed
        pitch: 1.0,       // Normal pitch
        reset: true,      // Reset state
    };

    debug!("Testing preroll with request: {:?}", request);
    stretcher.preroll(&mut request)?;
    info!("Preroll successful");

    // Get max input frame count
    let max_frames = stretcher.max_input_frame_count();
    info!("Max input frames: {}", max_frames);

    // Check flush state
    let is_flushed = stretcher.is_flushed();
    info!("Stretcher flushed state: {}", is_flushed);

    info!("Basic test completed successfully");
    Ok(())
} 