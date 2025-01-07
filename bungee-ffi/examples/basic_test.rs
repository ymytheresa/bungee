use bungee_ffi::{SampleRates, Request, Stretcher, bungee_output_chunk_t};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample rates configuration
    let rates = SampleRates {
        input: 44100,
        output: 44100,
    };

    // Create a new stretcher (stereo)
    let mut stretcher = Stretcher::new(rates, 2)?;

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

    // Configure initial request (1.5x slower)
    let mut request = Request {
        position: 0.0,
        speed: 1.5,    // 1.5x slower
        pitch: 1.0,    // Keep original pitch
        reset: true,   // Initial grain
    };

    // Prepare for processing
    stretcher.preroll(&mut request)?;

    // Create output buffer (1.5x size for time stretching)
    let output_size = (num_samples as f32 * 1.5) as usize;
    let mut output = vec![0.0f32; output_size];
    let mut output_pos = 0;

    // Process in grains
    while !stretcher.is_flushed() && output_pos < output_size {
        // Get required input range for this grain
        let (begin, end) = stretcher.specify_grain(&input, num_samples/2)?;
        
        // Skip if this is a flush grain
        if begin >= end {
            continue;
        }

        // Get the input slice for this grain
        let begin = begin as usize * 2; // *2 for stereo
        let end = end as usize * 2;     // *2 for stereo
        let input_slice = if begin < input.len() && end <= input.len() {
            &input[begin..end]
        } else {
            break;
        };

        // Analyze the grain
        stretcher.analyse_grain(input_slice, 1)?;

        // Prepare output chunk
        let remaining = output_size - output_pos;
        let mut output_chunk = bungee_output_chunk_t {
            data: output[output_pos..].as_mut_ptr(),
            frame_count: (remaining / 2) as i32,
            channel_stride: 1,
        };

        // Synthesize the grain
        stretcher.synthesise_grain(&mut output_chunk)?;
        output_pos += output_chunk.frame_count as usize * 2; // *2 for stereo

        // Prepare next grain
        request.reset = false;
        stretcher.next(&mut request)?;
    }

    println!("Processed {} samples into {} samples", num_samples, output_pos);
    println!("Time-stretched by factor of {}", request.speed);
    println!("Flushed: {}", stretcher.is_flushed());

    Ok(())
} 