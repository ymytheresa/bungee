use bungee_ffi::{Stretcher, SampleRates, Request};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting basic time-stretching test...");
    
    // Initialize the library
    bungee_ffi::init()?;
    println!("Library initialized");
    
    // Create a 1-second stereo sine wave at 440Hz
    let sample_rate: i32 = 44100;
    let num_samples = sample_rate as usize;  // 1 second
    let frequency = 440.0;  // A4 note
    let mut input = Vec::with_capacity(num_samples * 2);  // Stereo
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let value = (2.0 * std::f32::consts::PI * frequency * t).sin();
        // Left and right channels
        input.push(value);
        input.push(value);
    }
    
    println!("Created input: {} samples ({} seconds stereo)", num_samples, num_samples as f32 / sample_rate as f32);
    
    // Initialize stretcher with 1.5x speed
    let rates = SampleRates {
        input: sample_rate,
        output: sample_rate,
    };
    
    let mut stretcher = Stretcher::new(rates, 2)?;
    println!("Initialized stretcher");
    
    // Set request parameters
    let mut request = Request {
        position: 0.0,
        speed: 1.5,
        pitch: 1.0,
        reset: true,
    };
    
    // Preroll with initial request
    stretcher.preroll(&request)?;
    println!("Request configured: speed={}, pitch={}", request.speed, request.pitch);
    
    // Create output buffer (1.5x size for time stretching)
    let output_size = (num_samples as f32 * 1.5) as usize;
    let mut output = vec![0.0f32; output_size];
    let mut output_pos = 0;
    
    println!("\n=== Starting grain processing ===\n");
    
    // Process in grains
    while !stretcher.is_flushed() && output_pos < output_size {
        // Get required input range for this grain
        let (begin, end) = stretcher.specify_grain(&input, num_samples/2)?;
        println!("\nGrain boundaries: begin={}, end={}", begin, end);
        
        // Skip if this is a flush grain
        if begin >= end {
            println!("Skipping flush grain");
            continue;
        }
        
        // Get the input slice for this grain
        let begin = begin as usize * 2; // *2 for stereo
        let end = end as usize * 2;     // *2 for stereo
        let input_slice = if begin < input.len() && end <= input.len() {
            println!("Processing input slice: {}..{} ({} samples)", 
                    begin, end, (end - begin) / 2);
            &input[begin..end]
        } else {
            println!("Input slice out of bounds: {}..{}", begin, end);
            break;
        };
        
        // Check input slice for non-zero values
        let non_zero = input_slice.iter().any(|&x| x != 0.0);
        println!("Input contains non-zero values: {}", non_zero);
        
        // Analyze the grain
        stretcher.analyse_grain(input_slice, 1)?;
        
        // Prepare output chunk
        let remaining = output_size - output_pos;
        let mut output_chunk = bungee_ffi::bungee_output_chunk_t {
            data: output[output_pos..].as_mut_ptr(),
            frame_count: (remaining / 2) as i32,
            channel_stride: 2,
        };
        println!("Output chunk prepared: remaining={}, frame_count={}, channel_stride={}", 
                remaining, output_chunk.frame_count, output_chunk.channel_stride);
        
        // Synthesize the grain
        stretcher.synthesise_grain(&mut output_chunk)?;
        let frames_generated = output_chunk.frame_count as usize * 2; // *2 for stereo
        println!("Synthesized {} output frames", frames_generated / 2);
        
        // Check output for non-zero values
        let output_slice = &output[output_pos..output_pos + frames_generated];
        let non_zero_output = output_slice.iter().any(|&x| x != 0.0);
        println!("Output contains non-zero values: {}", non_zero_output);
        
        output_pos += frames_generated;
        
        // Prepare next grain
        request.reset = false;
        stretcher.next(&mut request)?;
        println!("Advanced to next grain: position={}", request.position);
    }
    
    println!("\n=== Processing complete ===");
    println!("Processed {} samples into {} samples", num_samples, output_pos);
    println!("Time-stretched by factor of {}", request.speed);
    println!("Flushed: {}", stretcher.is_flushed());
    
    // Analyze output
    let non_zero_samples = output[..output_pos].iter().filter(|&&x| x != 0.0).count();
    println!("Non-zero output samples: {}/{}", non_zero_samples, output_pos);
    
    if non_zero_samples == 0 {
        println!("\nWARNING: No non-zero samples in output!");
    }
    
    // Cleanup
    bungee_ffi::cleanup();
    
    Ok(())
} 