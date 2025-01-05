//! Safe Rust wrapper for the Bungee audio time stretching library
//! This module provides a safe interface to the Bungee library's functionality.

mod ffi;
mod error;

use std::ptr::NonNull;
use log::{debug, info};
pub use error::{Error, Result};

/// Safe wrapper for audio processing parameters
#[derive(Debug, Clone)]
pub struct Request {
    pub position: f64,  // Frame-offset within the input audio
    pub speed: f64,     // Output audio speed (1.0 = unchanged)
    pub pitch: f64,     // Frequency multiplier (1.0 = unchanged)
    pub reset: bool,    // Reset stretcher state
}

impl Request {
    /// Create a new request using semitones for pitch shifting
    /// 
    /// # Arguments
    /// * `position` - Frame-offset within the input audio
    /// * `speed` - Output audio speed (1.0 = unchanged)
    /// * `semitones` - Number of semitones to shift (positive = up, negative = down)
    /// * `reset` - Whether to reset stretcher state
    pub fn with_semitones(position: f64, speed: f64, semitones: f64, reset: bool) -> Self {
        Self {
            position,
            speed,
            pitch: 2.0f64.powf(semitones / 12.0),
            reset,
        }
    }

    /// Get the current pitch shift in semitones
    pub fn semitones(&self) -> f64 {
        12.0 * self.pitch.log2()
    }
}

/// Safe wrapper for input chunk specification
#[derive(Debug, Clone)]
pub struct InputChunk {
    pub begin: i32,  // Start frame offset
    pub end: i32,    // End frame offset
}

/// Safe wrapper for output chunk containing processed audio
#[derive(Debug)]
pub struct OutputChunk {
    pub data: Vec<f32>,          // Audio output data
    pub frame_count: i32,        // Number of frames
    pub channel_stride: isize,   // Stride between channels
    pub begin_request: Request,  // Request at start of chunk
    pub end_request: Request,    // Request at end of chunk
}

/// Main stretcher interface providing safe audio processing operations
pub struct Stretcher {
    // Raw pointer to the underlying Bungee stretcher instance
    inner: NonNull<std::ffi::c_void>,
    // Function table for operations
    vtable: ffi::Bungee_Stretcher_FunctionTable,
}

impl Stretcher {
    /// Create a new stretcher instance
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100)
    /// * `channels` - Number of audio channels (e.g., 2 for stereo)
    pub fn new(sample_rate: i32, channels: i32) -> Result<Self> {
        info!("Creating new Stretcher with sample_rate={}, channels={}", sample_rate, channels);
        
        debug!("Getting Bungee function table");
        let vtable = unsafe { ffi::Bungee_Stretcher_getFunctionTable() };
        
        debug!("Creating Bungee stretcher instance");
        let inner = (vtable.create)(
            ffi::Bungee_SampleRates {
                input: sample_rate,
                output: sample_rate,
            },
            channels,
            0, // default hop size
        );
        
        // Safety: create() is guaranteed to return a valid pointer
        let inner = NonNull::new(inner)
            .ok_or_else(|| Error::InitializationError("Bungee create() returned null".into()))?;
        
        info!("Successfully created Stretcher instance");
        Ok(Self { inner, vtable })
    }

    /// Prepare the stretcher for processing at the given position
    pub fn preroll(&mut self, request: &mut Request) -> Result<()> {
        debug!("Preroll called with request: {:?}", request);
        
        let mut ffi_request = ffi::Bungee_Request {
            position: request.position,
            speed: request.speed,
            pitch: request.pitch,
            reset: request.reset,
        };
        
        debug!("Calling Bungee preroll");
        (self.vtable.preroll)(self.inner.as_ptr(), &mut ffi_request);
        
        // Copy back any modifications
        request.position = ffi_request.position;
        debug!("Updated request position to {}", request.position);
        
        Ok(())
    }

    /// Get the maximum number of input frames that might be requested
    pub fn max_input_frame_count(&self) -> i32 {
        debug!("Getting max input frame count");
        let count = (self.vtable.max_input_frame_count)(self.inner.as_ptr());
        debug!("Max input frame count: {}", count);
        count
    }

    /// Check if all processing is complete
    pub fn is_flushed(&self) -> bool {
        debug!("Checking if stretcher is flushed");
        let flushed = (self.vtable.is_flushed)(self.inner.as_ptr());
        debug!("Stretcher flushed status: {}", flushed);
        flushed
    }

    /// Request the next grain of input audio needed for processing
    pub fn specify_grain(&mut self, request: &Request) -> Result<InputChunk> {
        debug!("Specifying grain with request: {:?}", request);
        
        let ffi_request = ffi::Bungee_Request::from(request.clone());
        let chunk = (self.vtable.specify_grain)(self.inner.as_ptr(), &ffi_request);
        
        debug!("Received input chunk: begin={}, end={}", chunk.begin, chunk.end);
        Ok(chunk.into())
    }

    /// Analyze a grain of input audio data
    pub fn analyse_grain(&mut self, data: &[f32], channel_stride: isize) -> Result<()> {
        debug!("Analyzing grain: {} samples, stride {}", data.len(), channel_stride);
        
        // Safety: data pointer is valid for the length of the slice
        (self.vtable.analyse_grain)(
            self.inner.as_ptr(),
            data.as_ptr(),
            channel_stride
        );
        
        debug!("Grain analysis complete");
        Ok(())
    }

    /// Synthesize the next grain of output audio
    pub fn synthesise_grain(&mut self, output: &mut OutputChunk) -> Result<()> {
        debug!("Synthesizing grain to output chunk: {:?}", output);
        
        // Create FFI requests - now using Copy trait and mut
        let mut ffi_begin_request = ffi::Bungee_Request::from(output.begin_request.clone());
        let mut ffi_end_request = ffi::Bungee_Request::from(output.end_request.clone());

        let mut ffi_chunk = ffi::Bungee_OutputChunk {
            data: output.data.as_mut_ptr(),
            frame_count: output.frame_count,
            channel_stride: output.channel_stride,
            request: [&mut ffi_begin_request, &mut ffi_end_request],
        };

        // Safety: output chunk is properly initialized
        (self.vtable.synthesise_grain)(self.inner.as_ptr(), &mut ffi_chunk);

        // Update requests with any modifications - now using Copy trait
        output.begin_request = ffi_begin_request.into();
        output.end_request = ffi_end_request.into();
        
        debug!("Grain synthesis complete");
        Ok(())
    }

    /// Advance to the next processing step
    pub fn next(&mut self, request: &mut Request) -> Result<()> {
        debug!("Advancing to next step with request: {:?}", request);
        
        let mut ffi_request = ffi::Bungee_Request::from(request.clone());
        (self.vtable.next)(self.inner.as_ptr(), &mut ffi_request);
        
        // Copy back any modifications
        *request = ffi_request.into();
        debug!("Advanced to next step, updated request: {:?}", request);
        Ok(())
    }

    /// Calculate the expected output duration given an input duration and request parameters
    /// 
    /// # Arguments
    /// * `input_duration` - Duration of input audio in seconds
    /// * `request` - Request parameters containing speed and pitch
    pub fn calculate_output_duration(&self, input_duration: f64, request: &Request) -> f64 {
        input_duration * request.speed / request.pitch
    }

    /// Calculate the expected output frame count given input frames and request parameters
    /// 
    /// # Arguments
    /// * `input_frames` - Number of input audio frames
    /// * `request` - Request parameters containing speed and pitch
    pub fn calculate_output_frames(&self, input_frames: usize, request: &Request) -> usize {
        (input_frames as f64 * request.speed / request.pitch) as usize
    }
}

// RAII: Clean up Bungee resources when Stretcher is dropped
impl Drop for Stretcher {
    fn drop(&mut self) {
        debug!("Dropping Stretcher instance");
        (self.vtable.destroy)(self.inner.as_ptr());
        debug!("Stretcher resources freed");
    }
}

// Implement conversion traits between FFI and safe types
impl From<ffi::Bungee_Request> for Request {
    fn from(req: ffi::Bungee_Request) -> Self {
        Self {
            position: req.position,
            speed: req.speed,
            pitch: req.pitch,
            reset: req.reset,
        }
    }
}

impl From<Request> for ffi::Bungee_Request {
    fn from(req: Request) -> Self {
        Self {
            position: req.position,
            speed: req.speed,
            pitch: req.pitch,
            reset: req.reset,
        }
    }
}

impl From<ffi::Bungee_InputChunk> for InputChunk {
    fn from(chunk: ffi::Bungee_InputChunk) -> Self {
        Self {
            begin: chunk.begin,
            end: chunk.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    use std::fs::File;
    use env_logger::Builder;
    use log::LevelFilter;

    /// Generate a test sine wave at the given frequency
    fn generate_test_sine(freq_hz: f32, sample_rate: i32, duration_secs: f32) -> Vec<f32> {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let angular_freq = 2.0 * PI * freq_hz;
        let dt = 1.0 / sample_rate as f32;
        
        (0..num_samples)
            .map(|i| (angular_freq * i as f32 * dt).sin())
            .collect()
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

    #[test]
    fn test_batch_processing() {
        // Initialize logging to file
        let log_path = "../test.log";
        let file = File::create(log_path).expect("Failed to create log file");
        Builder::new()
            .target(env_logger::Target::Pipe(Box::new(file)))
            .filter_level(LevelFilter::Debug)
            .init();

        info!("Starting batch processing test");
        
        // Test parameters
        let sample_rate = 44100;
        let channels = 1;
        let input_freq_hz = 440.0; // A4 note
        let duration_secs = 0.1;  // Shorter test duration
        let pitch_shift = 2.0; // Shift up one octave

        info!("Generating test signal: {}Hz sine wave, {}s duration", input_freq_hz, duration_secs);
        let input_audio = generate_test_sine(input_freq_hz, sample_rate, duration_secs);
        info!("Generated {} samples", input_audio.len());
        
        // Create stretcher with pitch shift
        let mut stretcher = Stretcher::new(sample_rate, channels).expect("Failed to create stretcher");
        
        // Initialize processing request
        let mut request = Request {
            position: 0.0,
            speed: 1.0,
            pitch: pitch_shift,
            reset: true,
        };

        // Check initial state
        let max_input_frames = stretcher.max_input_frame_count();
        info!("Max input frames: {}", max_input_frames);
        info!("Initial flushed state: {}", stretcher.is_flushed());

        // Preroll the stretcher
        stretcher.preroll(&mut request).expect("Failed to preroll");
        info!("Initialized stretcher: speed={}, pitch={}", request.speed, request.pitch);
        info!("Flushed state after preroll: {}", stretcher.is_flushed());

        // Try to get first chunk
        let first_chunk = stretcher.specify_grain(&request).expect("Failed to specify first grain");
        info!("First chunk: begin={}, end={}", first_chunk.begin, first_chunk.end);

        let mut output_audio = Vec::new();
        let mut total_chunks = 0;
        
        // Process audio in grains
        while !stretcher.is_flushed() {
            // Get next input chunk needed
            let chunk = stretcher.specify_grain(&request).expect("Failed to specify grain");
            debug!("Processing chunk {}: begin={}, end={}", total_chunks + 1, chunk.begin, chunk.end);
            
            // Get padded grain data
            let grain_data = get_padded_grain(&input_audio, chunk.begin, chunk.end);
            debug!("Grain size: {}", grain_data.len());

            // Analyze the grain
            stretcher.analyse_grain(&grain_data, 1).expect("Failed to analyze grain");

            // Prepare output chunk
            let max_frames = 1024; // Reasonable buffer size
            let mut output_chunk = OutputChunk {
                data: vec![0.0; max_frames],
                frame_count: max_frames as i32,
                channel_stride: 1,
                begin_request: request.clone(),
                end_request: request.clone(),
            };

            // Synthesize output
            stretcher.synthesise_grain(&mut output_chunk).expect("Failed to synthesize grain");
            debug!("Generated {} output frames", output_chunk.frame_count);

            // Collect output audio
            output_audio.extend_from_slice(&output_chunk.data[..output_chunk.frame_count as usize]);

            // Advance to next step
            stretcher.next(&mut request).expect("Failed to advance to next step");
            debug!("Advanced to position: {}", request.position);
            
            total_chunks += 1;
            if total_chunks % 100 == 0 {
                info!("Processed {} chunks, position: {}", total_chunks, request.position);
            }
        }

        info!("Processing complete: {} chunks processed", total_chunks);
        info!("Input samples: {}", input_audio.len());
        info!("Output samples: {}", output_audio.len());
        info!("Output length ratio: {:.2}", output_audio.len() as f32 / input_audio.len() as f32);

        // Basic quality checks
        assert!(!output_audio.is_empty(), "No output audio generated");
        
        // Check if output length is reasonable (roughly same as input)
        let length_ratio = output_audio.len() as f32 / input_audio.len() as f32;
        assert!((0.8..1.2).contains(&length_ratio), "Output length ratio out of bounds: {}", length_ratio);

        // Check if output is non-zero and within bounds
        let max_amplitude = output_audio.iter().fold(0.0f32, |max, &x| max.max(x.abs()));
        assert!(max_amplitude > 0.0, "Output audio is silent");
        assert!(max_amplitude <= 1.0, "Output audio is clipping");

        info!("All quality checks passed");
        // TODO: Add frequency analysis to verify pitch shift
    }
}
