#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod error;

use std::ptr::NonNull;
pub use error::BungeeError;

// Include the bindgen generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SampleRates {
    pub input: i32,
    pub output: i32,
}

impl From<SampleRates> for bungee_sample_rates_t {
    fn from(rates: SampleRates) -> Self {
        Self {
            input_rate: rates.input,
            output_rate: rates.output,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Request {
    pub position: f64,
    pub speed: f64,
    pub pitch: f64,
    pub reset: bool,
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

#[derive(Debug)]
pub struct Stretcher {
    inner: NonNull<bungee_stretcher_t>,
}

/// Initialize the Bungee library
pub fn init() -> Result<(), BungeeError> {
    let result = unsafe {
        bungee_init()
    };
    
    if result == 0 {  // BUNGEE_OK
        Ok(())
    } else {
        Err(result.into())
    }
}

/// Clean up the Bungee library
pub fn cleanup() {
    unsafe {
        bungee_cleanup();
    }
}

impl Stretcher {
    /// Create a new stretcher instance
    pub fn new(rates: SampleRates, channels: i32) -> Result<Self, BungeeError> {
        let inner = unsafe {
            let ptr = bungee_create(rates.into(), channels);
            NonNull::new(ptr).ok_or(BungeeError::Memory)?
        };
        Ok(Self { inner })
    }

    /// Prepare for processing with initial parameters
    pub fn preroll(&mut self, request: &Request) -> Result<(), BungeeError> {
        let c_request = bungee_request_t::from(*request);
        let result = unsafe {
            bungee_preroll(self.inner.as_ptr(), &c_request)
        };
        
        if result == 0 {  // BUNGEE_OK
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

    /// Advance to the next grain
    pub fn next(&mut self, request: &mut Request) -> Result<(), BungeeError> {
        let mut c_request = bungee_request_t::from(*request);
        let result = unsafe {
            bungee_next(self.inner.as_ptr(), &mut c_request)
        };
        
        if result == 0 {  // BUNGEE_OK
            *request = Request {
                position: c_request.position,
                speed: c_request.speed,
                pitch: c_request.pitch,
                reset: c_request.reset,
            };
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