#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod error;

use std::ptr::NonNull;
pub use error::BungeeError;

// Include the bindgen generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// A safe wrapper around bungee_stretcher_t
pub struct Stretcher {
    inner: NonNull<bungee_stretcher_t>,
}

/// Sample rates configuration
#[derive(Debug, Clone)]
pub struct SampleRates {
    pub input: i32,
    pub output: i32,
}

/// Request configuration for a grain
#[derive(Debug, Clone)]
pub struct Request {
    pub position: f64,  // Frame offset in input audio
    pub speed: f64,     // 1.0 = normal speed
    pub pitch: f64,     // 1.0 = normal pitch
    pub reset: bool,    // Reset stretcher state
}

impl From<Request> for bungee_request_t {
    fn from(req: Request) -> Self {
        Self {
            position: req.position,
            speed: req.speed,
            pitch: req.pitch,
            reset: req.reset,
        }
    }
}

impl From<SampleRates> for bungee_sample_rates_t {
    fn from(rates: SampleRates) -> Self {
        Self {
            input_rate: rates.input,
            output_rate: rates.output,
        }
    }
}

impl Stretcher {
    /// Create a new stretcher instance
    pub fn new(rates: SampleRates, channels: i32) -> Result<Self, BungeeError> {
        // Initialize the library if needed
        unsafe {
            if bungee_init() != 0 {
                return Err(BungeeError::InvalidState);
            }
        }
        
        // Create the stretcher
        let ptr = unsafe { 
            bungee_create(rates.into(), channels)
        };
        
        NonNull::new(ptr)
            .map(|inner| Self { inner })
            .ok_or(BungeeError::NullPointer)
    }

    /// Get the maximum number of input frames that might be requested
    pub fn max_input_frame_count(&self) -> usize {
        unsafe {
            bungee_max_input_frame_count(self.inner.as_ptr())
        }
    }

    /// Prepare request position for initial processing
    pub fn preroll(&self, request: &mut Request) -> Result<(), BungeeError> {
        let mut c_request: bungee_request_t = request.clone().into();
        
        // SAFETY: self.inner and request are guaranteed to be valid
        let result = unsafe {
            bungee_preroll(self.inner.as_ptr(), &mut c_request)
        };
        
        if result == 0 {  // BUNGEE_OK
            request.position = c_request.position;
            Ok(())
        } else {
            Err(result.into())
        }
    }

    /// Prepare request for the next grain
    pub fn next(&self, request: &mut Request) -> Result<(), BungeeError> {
        let mut c_request: bungee_request_t = request.clone().into();
        
        // SAFETY: self.inner and request are guaranteed to be valid
        let result = unsafe {
            bungee_next(self.inner.as_ptr(), &mut c_request)
        };
        
        if result == 0 {  // BUNGEE_OK
            request.position = c_request.position;
            Ok(())
        } else {
            Err(result.into())
        }
    }

    /// Specify a grain of audio and get required input range
    pub fn specify_grain(&mut self, input: &[f32], frame_count: usize) -> Result<(i32, i32), BungeeError> {
        let mut chunk = bungee_input_chunk_t {
            begin: 0,
            end: 0,
        };
        
        let result = unsafe {
            bungee_specify_grain(
                self.inner.as_ptr(),
                input.as_ptr(),
                frame_count,
                &mut chunk
            )
        };
        
        if result == 0 {  // BUNGEE_OK
            Ok((chunk.begin, chunk.end))
        } else {
            Err(result.into())
        }
    }

    /// Analyze the current grain
    pub fn analyse_grain(&mut self, data: &[f32], channel_stride: usize) -> Result<(), BungeeError> {
        let result = unsafe {
            bungee_analyse_grain(
                self.inner.as_ptr(),
                data.as_ptr(),
                channel_stride,
            )
        };
        
        if result == 0 {  // BUNGEE_OK
            Ok(())
        } else {
            Err(result.into())
        }
    }

    /// Synthesize the processed grain
    pub fn synthesise_grain(&mut self, output: &mut bungee_output_chunk_t) -> Result<(), BungeeError> {
        let result = unsafe {
            bungee_synthesise_grain(self.inner.as_ptr(), output)
        };
        
        if result == 0 {  // BUNGEE_OK
            Ok(())
        } else {
            Err(result.into())
        }
    }

    /// Check if all grains have been processed
    pub fn is_flushed(&self) -> bool {
        unsafe {
            bungee_is_flushed(self.inner.as_ptr())
        }
    }
}

impl Drop for Stretcher {
    fn drop(&mut self) {
        unsafe {
            bungee_destroy(self.inner.as_ptr());
        }
    }
}

// Implement Send and Sync for Stretcher as it's thread-safe
unsafe impl Send for Stretcher {}
unsafe impl Sync for Stretcher {} 