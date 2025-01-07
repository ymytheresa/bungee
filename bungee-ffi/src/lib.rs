#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod error;

use std::ptr::NonNull;
use thiserror::Error;

pub use error::BungeeError;

// Include the bindgen generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// A safe wrapper around Bungee_Stretcher
pub struct Stretcher {
    inner: NonNull<std::ffi::c_void>,
    vtable: Bungee_Stretcher_FunctionTable,
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

impl From<Request> for Bungee_Request {
    fn from(req: Request) -> Self {
        Self {
            position: req.position,
            speed: req.speed,
            pitch: req.pitch,
            reset: req.reset,
        }
    }
}

impl From<SampleRates> for Bungee_SampleRates {
    fn from(rates: SampleRates) -> Self {
        Self {
            input: rates.input,
            output: rates.output,
        }
    }
}

impl Stretcher {
    /// Create a new stretcher instance using the Basic implementation
    pub fn new(rates: SampleRates, channels: i32) -> Result<Self, BungeeError> {
        // SAFETY: This is a C function that returns a struct of function pointers
        let vtable = unsafe { Bungee_Stretcher_getFunctionTable() };
        
        // SAFETY: We pass valid parameters and check the result
        let ptr = unsafe { 
            (vtable.create)(rates.into(), channels, 0)
        };
        
        NonNull::new(ptr)
            .map(|inner| Self { inner, vtable })
            .ok_or(BungeeError::NullPointer)
    }

    /// Create a new stretcher instance using the Pro implementation
    pub fn new_pro(rates: SampleRates, channels: i32) -> Result<Self, BungeeError> {
        // SAFETY: This is a C function that returns a struct of function pointers
        let vtable = unsafe { BungeePro_Stretcher_getFunctionTable() };
        
        // SAFETY: We pass valid parameters and check the result
        let ptr = unsafe { 
            (vtable.create)(rates.into(), channels, 0)
        };
        
        NonNull::new(ptr)
            .map(|inner| Self { inner, vtable })
            .ok_or(BungeeError::NullPointer)
    }

    /// Get the maximum number of input frames that might be requested
    pub fn max_input_frame_count(&self) -> i32 {
        // SAFETY: self.inner is guaranteed to be valid
        unsafe {
            (self.vtable.maxInputFrameCount)(self.inner.as_ptr())
        }
    }

    /// Prepare request position for initial processing
    pub fn preroll(&self, request: &mut Request) {
        let mut c_request: Bungee_Request = request.clone().into();
        // SAFETY: self.inner and request are guaranteed to be valid
        unsafe {
            (self.vtable.preroll)(self.inner.as_ptr(), &mut c_request);
        }
        request.position = c_request.position;
    }

    /// Prepare request for the next grain
    pub fn next(&self, request: &mut Request) {
        let mut c_request: Bungee_Request = request.clone().into();
        // SAFETY: self.inner and request are guaranteed to be valid
        unsafe {
            (self.vtable.next)(self.inner.as_ptr(), &mut c_request);
        }
        request.position = c_request.position;
    }

    /// Specify a grain of audio and get required input range
    pub fn specify_grain(&mut self, request: &Request) -> (i32, i32) {
        // SAFETY: self.inner and request are guaranteed to be valid
        let chunk = unsafe {
            (self.vtable.specifyGrain)(self.inner.as_ptr(), &request.clone().into())
        };
        (chunk.begin, chunk.end)
    }

    /// Analyze the current grain
    pub fn analyse_grain(&mut self, data: &[f32], channel_stride: isize) {
        // SAFETY: self.inner and data are guaranteed to be valid
        unsafe {
            (self.vtable.analyseGrain)(
                self.inner.as_ptr(),
                data.as_ptr(),
                channel_stride,
            );
        }
    }

    /// Synthesize the processed grain
    pub fn synthesise_grain(&mut self, output: &mut Bungee_OutputChunk) {
        // SAFETY: self.inner and output are guaranteed to be valid
        unsafe {
            (self.vtable.synthesiseGrain)(self.inner.as_ptr(), output);
        }
    }

    /// Check if all grains have been processed
    pub fn is_flushed(&self) -> bool {
        // SAFETY: self.inner is guaranteed to be valid
        unsafe {
            (self.vtable.isFlushed)(self.inner.as_ptr())
        }
    }

    /// Get the library version
    pub fn version(&self) -> &str {
        // SAFETY: This returns a null-terminated C string
        unsafe {
            let c_str = std::ffi::CStr::from_ptr((self.vtable.version)());
            c_str.to_str().unwrap_or("unknown")
        }
    }
}

impl Drop for Stretcher {
    fn drop(&mut self) {
        // SAFETY: self.inner is guaranteed to be non-null and valid
        unsafe {
            (self.vtable.destroy)(self.inner.as_ptr());
        }
    }
}

// Implement Send and Sync for Stretcher as it's thread-safe
unsafe impl Send for Stretcher {}
unsafe impl Sync for Stretcher {} 