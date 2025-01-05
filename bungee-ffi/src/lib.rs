//! Safe Rust wrapper for the Bungee audio time stretching library
//! This module provides a safe interface to the Bungee library's functionality.

mod ffi;
mod error;

use std::ptr::NonNull;
use log::{debug, info, warn};
pub use error::{Error, Result};

/// Safe wrapper for audio processing parameters
#[derive(Debug, Clone)]
pub struct Request {
    pub position: f64,  // Frame-offset within the input audio
    pub speed: f64,     // Output audio speed (1.0 = unchanged)
    pub pitch: f64,     // Frequency multiplier (1.0 = unchanged)
    pub reset: bool,    // Reset stretcher state
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
        
        unsafe {
            debug!("Getting Bungee function table");
            let vtable = ffi::Bungee_Stretcher_getFunctionTable();
            
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
    }

    /// Prepare the stretcher for processing at the given position
    pub fn preroll(&mut self, request: &mut Request) -> Result<()> {
        debug!("Preroll called with request: {:?}", request);
        
        unsafe {
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
    }

    /// Get the maximum number of input frames that might be requested
    pub fn max_input_frame_count(&self) -> i32 {
        debug!("Getting max input frame count");
        unsafe {
            let count = (self.vtable.max_input_frame_count)(self.inner.as_ptr());
            debug!("Max input frame count: {}", count);
            count
        }
    }

    /// Check if all processing is complete
    pub fn is_flushed(&self) -> bool {
        debug!("Checking if stretcher is flushed");
        unsafe {
            let flushed = (self.vtable.is_flushed)(self.inner.as_ptr());
            debug!("Stretcher flushed status: {}", flushed);
            flushed
        }
    }
}

// RAII: Clean up Bungee resources when Stretcher is dropped
impl Drop for Stretcher {
    fn drop(&mut self) {
        debug!("Dropping Stretcher instance");
        unsafe {
            (self.vtable.destroy)(self.inner.as_ptr());
        }
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
