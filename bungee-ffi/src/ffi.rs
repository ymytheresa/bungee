//! Raw FFI bindings to the Bungee library
//! This module contains unsafe FFI declarations and should not be used directly.
//! Use the safe wrapper types from the root module instead.

use std::ffi::c_void;
use std::mem::{size_of, align_of};
use log::{debug, warn};
use memoffset;

/// Sample rates for input and output
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bungee_SampleRates {
    pub input: i32,
    pub output: i32,
}

/// Request parameters for time stretching
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bungee_Request {
    pub position: f64,  // Frame-offset within the input audio
    pub speed: f64,     // Output audio speed (1.0 = unchanged)
    pub pitch: f64,     // Frequency multiplier (1.0 = unchanged)
    pub reset: bool,    // Reset stretcher state
}

/// Input chunk specification
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bungee_InputChunk {
    pub begin: i32,  // Start frame offset
    pub end: i32,    // End frame offset
}

/// Output chunk containing processed audio data
#[repr(C)]
pub struct Bungee_OutputChunk {
    pub data: *mut f32,           // Audio output data buffer
    pub frame_count: i32,         // Number of frames
    pub channel_stride: isize,    // Stride between channels
    pub request: [*mut Bungee_Request; 2],  // Begin/end requests
}

/// Function table containing all operations provided by the Bungee library
#[repr(C)]
pub struct Bungee_Stretcher_FunctionTable {
    pub version: extern "C" fn() -> *const i8,
    pub create: extern "C" fn(Bungee_SampleRates, i32, i32) -> *mut c_void,
    pub destroy: extern "C" fn(*mut c_void),
    pub max_input_frame_count: extern "C" fn(*const c_void) -> i32,
    pub preroll: extern "C" fn(*const c_void, *mut Bungee_Request),
    pub next: extern "C" fn(*const c_void, *mut Bungee_Request),
    pub specify_grain: extern "C" fn(*mut c_void, *const Bungee_Request) -> Bungee_InputChunk,
    pub analyse_grain: extern "C" fn(*mut c_void, *const f32, isize),
    pub synthesise_grain: extern "C" fn(*mut c_void, *mut Bungee_OutputChunk),
    pub is_flushed: extern "C" fn(*const c_void) -> bool,
}

// Validate FFI struct sizes match C++ side
const fn validate_ffi_sizes() {
    // C++ struct sizes (from sizeof())
    const CPP_REQUEST_SIZE: usize = 32;      // sizeof(Bungee_Request)
    const CPP_INPUT_CHUNK_SIZE: usize = 8;   // sizeof(Bungee_InputChunk)
    const CPP_OUTPUT_CHUNK_SIZE: usize = 32; // sizeof(Bungee_OutputChunk)
    
    // C++ struct alignments
    const CPP_REQUEST_ALIGN: usize = 8;      // alignof(Bungee_Request)
    const CPP_INPUT_CHUNK_ALIGN: usize = 4;  // alignof(Bungee_InputChunk)
    const CPP_OUTPUT_CHUNK_ALIGN: usize = 8; // alignof(Bungee_OutputChunk)
    
    // Validate sizes
    assert!(size_of::<Bungee_Request>() == CPP_REQUEST_SIZE, "Bungee_Request size mismatch");
    assert!(size_of::<Bungee_InputChunk>() == CPP_INPUT_CHUNK_SIZE, "Bungee_InputChunk size mismatch");
    assert!(size_of::<Bungee_OutputChunk>() == CPP_OUTPUT_CHUNK_SIZE, "Bungee_OutputChunk size mismatch");
    
    // Validate alignments
    assert!(align_of::<Bungee_Request>() == CPP_REQUEST_ALIGN, "Bungee_Request alignment mismatch");
    assert!(align_of::<Bungee_InputChunk>() == CPP_INPUT_CHUNK_ALIGN, "Bungee_InputChunk alignment mismatch");
    assert!(align_of::<Bungee_OutputChunk>() == CPP_OUTPUT_CHUNK_ALIGN, "Bungee_OutputChunk alignment mismatch");
}

// Runtime validation for FFI struct layouts
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_ffi_layouts() {
        // Verify Request field offsets
        assert_eq!(memoffset::offset_of!(Bungee_Request, position), 0);
        assert_eq!(memoffset::offset_of!(Bungee_Request, speed), 8);
        assert_eq!(memoffset::offset_of!(Bungee_Request, pitch), 16);
        assert_eq!(memoffset::offset_of!(Bungee_Request, reset), 24);
        
        // Verify InputChunk field offsets
        assert_eq!(memoffset::offset_of!(Bungee_InputChunk, begin), 0);
        assert_eq!(memoffset::offset_of!(Bungee_InputChunk, end), 4);
        
        // Verify OutputChunk field offsets
        assert_eq!(memoffset::offset_of!(Bungee_OutputChunk, data), 0);
        assert_eq!(memoffset::offset_of!(Bungee_OutputChunk, frame_count), 8);
        assert_eq!(memoffset::offset_of!(Bungee_OutputChunk, channel_stride), 16);
        assert_eq!(memoffset::offset_of!(Bungee_OutputChunk, request), 24);
    }
}

// Debug logging for FFI parameters
impl Bungee_Request {
    pub fn log_debug(&self, prefix: &str) {
        debug!("{}Bungee_Request:", prefix);
        debug!("  position: {}", self.position);
        debug!("  speed: {}", self.speed);
        debug!("  pitch: {}", self.pitch);
        debug!("  reset: {}", self.reset);
    }
}

impl Bungee_InputChunk {
    pub fn log_debug(&self, prefix: &str) {
        debug!("{}Bungee_InputChunk:", prefix);
        debug!("  begin: {}", self.begin);
        debug!("  end: {}", self.end);
    }
}

impl Bungee_OutputChunk {
    pub fn log_debug(&self, prefix: &str) {
        debug!("{}Bungee_OutputChunk:", prefix);
        debug!("  frame_count: {}", self.frame_count);
        debug!("  channel_stride: {}", self.channel_stride);
        if let Some(req) = unsafe { self.request[0].as_ref() } {
            req.log_debug("  begin_request: ");
        }
        if let Some(req) = unsafe { self.request[1].as_ref() } {
            req.log_debug("  end_request: ");
        }
    }
}

extern "C" {
    /// Get the function table for the basic Bungee implementation
    fn _Bungee_Stretcher_getFunctionTable() -> Bungee_Stretcher_FunctionTable;
}

// Add debug logging to FFI functions
pub unsafe fn Bungee_Stretcher_getFunctionTable() -> Bungee_Stretcher_FunctionTable {
    debug!("Getting Bungee function table");
    let table = _Bungee_Stretcher_getFunctionTable();
    debug!("Got function table");
    table
} 