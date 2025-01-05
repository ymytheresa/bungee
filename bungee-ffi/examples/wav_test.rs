use bungee_ffi::{Stretcher, Request, OutputChunk};
use hound::{WavReader, WavWriter, SampleFormat};
use std::path::Path;
use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(name = "wav_test")]
#[command(about = "Process audio with pitch shifting")]
struct Args {
    /// Process full duration if true, otherwise only process 30 seconds
    #[arg(long, default_value_t = false)]
    full_duration: bool,

    /// Pitch shift in semitones (e.g., 12 for octave up, -12 for octave down)
    #[arg(long, default_value_t = 12.0)]
    semitones: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Initialize logging
    env_logger::init();

    // Input WAV file path (using absolute paths)
    let input_path = Path::new("/Users/bubu/Doc/Github/bungee/audio/Instrumental - Heart - 190.5bpm - Emaj.wav");
    let output_path = Path::new("/Users/bubu/Doc/Github/bungee/audio/output_pitched.wav");

    println!("Reading input WAV file...");
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();
    let input_duration = reader.duration() as f32 / spec.sample_rate as f32;
    println!("Input specs:");
    println!("  - Duration: {} seconds", input_duration);
    println!("  - Sample rate: {} Hz", spec.sample_rate);
    println!("  - Channels: {}", spec.channels);
    println!("  - Sample format: {:?}", spec.sample_format);

    // Convert samples to f32, properly handling 16-bit samples
    let samples: Vec<f32> = match spec.sample_format {
        SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
        SampleFormat::Int => reader.samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect(),
    };
    println!("Loaded {} samples", samples.len());

    let mut stretcher = Stretcher::new(
        spec.sample_rate.try_into().unwrap(),
        spec.channels as i32
    )?;

    // Initialize request with pitch shift using semitones
    let mut request = Request::with_semitones(
        0.0,            // start position
        1.0,            // normal speed
        args.semitones, // pitch shift in semitones
        true,           // reset state
    );

    println!("Prerolling stretcher...");
    println!("Pitch settings:");
    println!("  - Semitones: {}", args.semitones);
    println!("  - Pitch multiplier: {}", request.pitch);
    stretcher.preroll(&mut request)?;

    // Create output WAV file with same format as input
    let mut writer = WavWriter::create(output_path, spec)?;

    // Process audio in chunks
    let mut output_audio = Vec::new();
    let mut processed_frames = 0;
    let mut total_output_frames = 0;
    let total_frames = samples.len() / spec.channels as usize;
    
    // Calculate target frames and duration
    let target_duration = if args.full_duration {
        input_duration as f64
    } else {
        30.0
    };
    
    let target_frames = (target_duration * spec.sample_rate as f64) as usize;
    let expected_output_duration = stretcher.calculate_output_duration(target_duration, &request);
    let expected_output_frames = stretcher.calculate_output_frames(target_frames, &request);
    
    println!("\nProcessing configuration:");
    println!("  - Target duration: {} seconds", target_duration);
    println!("  - Target frames: {}", target_frames);
    println!("  - Expected output duration: {} seconds", expected_output_duration);
    println!("  - Expected output frames: {}", expected_output_frames);
    println!("  - Full duration mode: {}", args.full_duration);
    println!("  - Speed: {}", request.speed);
    println!("  - Pitch: {} ({})", request.pitch, request.semitones());
    
    let mut chunk_count = 0;
    println!("\nStarting audio processing...");
    
    // Process until we've handled all input frames or reached duration limit
    while processed_frames < total_frames && processed_frames < target_frames {
        chunk_count += 1;
        let chunk = stretcher.specify_grain(&request)?;
        println!("Chunk {}: Processing frames {} to {} (size: {})", 
            chunk_count, chunk.begin, chunk.end, chunk.end - chunk.begin);
        
        let grain_data = get_padded_grain(&samples, chunk.begin, chunk.end);
        stretcher.analyse_grain(&grain_data, spec.channels as isize)?;

        let chunk_max_frames = 1024;
        let mut output_chunk = OutputChunk {
            data: vec![0.0; chunk_max_frames * spec.channels as usize],
            frame_count: chunk_max_frames as i32,
            channel_stride: spec.channels as isize,
            begin_request: request.clone(),
            end_request: request.clone(),
        };

        stretcher.synthesise_grain(&mut output_chunk)?;

        let output_frames = output_chunk.frame_count as usize * spec.channels as usize;
        if output_frames > 0 {
            output_audio.extend_from_slice(&output_chunk.data[..output_frames]);
            total_output_frames += output_chunk.frame_count as usize;
        }

        stretcher.next(&mut request)?;
        
        let chunk_frames = if chunk.end > 0 {
            chunk.end as usize - chunk.begin.max(0) as usize
        } else {
            0
        };
        processed_frames += chunk_frames;

        // Show progress every 10%
        if processed_frames % (target_frames / 10) == 0 {
            println!("Progress: {}% ({}/{} frames)", 
                (processed_frames * 100) / target_frames,
                processed_frames,
                target_frames
            );
        }
    }

    let output_duration = output_audio.len() as f32 / (spec.sample_rate * spec.channels as u32) as f32;
    println!("\nProcessing complete:");
    println!("  - Processed {} chunks", chunk_count);
    println!("  - Input frames processed: {}", processed_frames);
    println!("  - Output frames generated: {}", total_output_frames);
    println!("  - Output duration: {} seconds", output_duration);
    println!("  - Target duration: {} seconds", target_duration);
    println!("  - Expected output duration: {} seconds", expected_output_duration);
    
    // Validate output duration
    let duration_error = (output_duration as f64 - expected_output_duration).abs();
    let is_duration_correct = duration_error < 1.0; // Allow 1 second tolerance
    
    println!("\nWriting output file...");
    // Convert back to the original sample format
    match spec.sample_format {
        SampleFormat::Float => {
            for sample in output_audio {
                writer.write_sample(sample)?;
            }
        },
        SampleFormat::Int => {
            for sample in output_audio {
                // Clamp samples to [-1.0, 1.0] to prevent distortion
                let clamped = sample.max(-1.0).min(1.0);
                let int_sample = (clamped * i16::MAX as f32) as i16;
                writer.write_sample(int_sample)?;
            }
        }
    }
    writer.finalize()?;

    println!("Output written to: {}", output_path.display());
    
    // Write bug report if duration is incorrect
    if !is_duration_correct {
        let bug_report = format!(r#"# Audio Processing Duration Bug

## Configuration
- Full duration mode: {}
- Target duration: {} seconds
- Speed: {}
- Pitch shift: {} semitones (multiplier: {})

## Problem
The output duration ({} seconds) does not match the expected duration ({} seconds).
Duration error: {} seconds

## Processing Details
- Input duration: {} seconds
- Sample rate: {} Hz
- Channels: {}
- Total input frames: {}
- Processed frames: {}
- Output frames: {}
- Expected output frames: {}
- Chunks processed: {}

## Analysis
The output duration is significantly different than expected. This might indicate:
1. Frames are being dropped during processing
2. The stretcher is not generating the expected number of output frames
3. There might be an issue with the frame counting logic

## Steps to Reproduce
1. Run wav_test with{} --full-duration flag --semitones {}
2. Process {} second audio file
3. Check output duration
"#,
            args.full_duration,
            target_duration,
            request.speed,
            request.semitones(),
            request.pitch,
            output_duration,
            expected_output_duration,
            duration_error,
            input_duration,
            spec.sample_rate,
            spec.channels,
            total_frames,
            processed_frames,
            total_output_frames,
            expected_output_frames,
            chunk_count,
            if args.full_duration { "" } else { "out" },
            args.semitones,
            if args.full_duration { input_duration } else { 30.0 }
        );
        
        fs::write("../bug.md", bug_report)?;
        println!("\n❌ Duration validation failed! Check bug.md for details.");
    } else {
        println!("\n✅ Duration validation passed!");
        // Clear any existing bug report since the test passed
        if Path::new("../bug.md").exists() {
            fs::remove_file("../bug.md")?;
        }
    }

    Ok(())
}

/// Get grain data with zero padding for negative indices
fn get_padded_grain(input: &[f32], begin: i32, end: i32) -> Vec<f32> {
    let mut grain = Vec::with_capacity((end - begin) as usize);
    
    // Handle the portion before input start (pad with zeros)
    let zeros_count = (-begin).max(0) as usize;
    grain.extend(std::iter::repeat(0.0).take(zeros_count));
    
    // Handle the portion within input
    let input_start = begin.max(0) as usize;
    let input_end = end.min(input.len() as i32) as usize;
    if input_start < input_end {
        grain.extend_from_slice(&input[input_start..input_end]);
    }
    
    // Handle the portion after input end (pad with zeros)
    let remaining = (end as usize).saturating_sub(input_end);
    grain.extend(std::iter::repeat(0.0).take(remaining));
    
    grain
} 