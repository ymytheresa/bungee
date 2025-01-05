//! Raw FFI bindings to the Bungee library
//! This module contains unsafe FFI declarations and should not be used directly.
//! Use the safe wrapper types from the root module instead.

use std::ffi::c_void;

/// Sample rates for input and output
#[repr(C)]
pub struct Bungee_SampleRates {
    pub input: i32,
    pub output: i32,
}

/// Request parameters for time stretching
#[repr(C)]
pub struct Bungee_Request {
    pub position: f64,  // Frame-offset within the input audio
    pub speed: f64,     // Output audio speed (1.0 = unchanged)
    pub pitch: f64,     // Frequency multiplier (1.0 = unchanged)
    pub reset: bool,    // Reset stretcher state
}

/// Input chunk specification
#[repr(C)]
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

extern "C" {
    /// Get the function table for the basic Bungee implementation
    pub fn Bungee_Stretcher_getFunctionTable() -> Bungee_Stretcher_FunctionTable;
} 