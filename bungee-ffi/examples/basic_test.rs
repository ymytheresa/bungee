use bungee_ffi::{Config, Stretcher};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple configuration
    let config = Config {
        sample_rate: 44100,
        channels: 2,
        time_ratio: 1.5, // 1.5x slower
        pitch_scale: 1.0, // Keep original pitch
    };

    // Create a new stretcher
    let mut stretcher = Stretcher::new(config)?;

    // Create some dummy input data (1 second of 440Hz sine wave)
    let sample_rate = 44100;
    let freq = 440.0;
    let duration = 1.0;
    let num_samples = (sample_rate as f32 * duration) as usize * 2; // *2 for stereo
    let mut input = vec![0.0f32; num_samples];
    
    for i in 0..num_samples/2 {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
        input[i*2] = sample;     // Left channel
        input[i*2+1] = sample;   // Right channel
    }

    // Create output buffer (1.5x size for time stretching)
    let output_size = (num_samples as f32 * 1.5) as usize;
    let mut output = vec![0.0f32; output_size];

    // Process the audio
    let (frames_used, frames_generated) = stretcher.process(&input, &mut output)?;
    println!("Processed {} input frames into {} output frames", frames_used, frames_generated);

    // Flush any remaining samples
    let additional_frames = stretcher.flush(&mut output[frames_generated*2..])?;
    println!("Flushed {} additional frames", additional_frames);

    Ok(())
} 