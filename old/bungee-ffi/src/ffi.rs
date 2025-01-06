//! Raw FFI bindings to the Bungee library
//! This module contains unsafe FFI declarations and should not be used directly.
//! Use the safe wrapper types from the root module instead.

use std::ffi::c_void;
use std::os::raw::{c_float, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum BungeeError {
    Ok = 0,
    NullPointer,
    InvalidParam,
    Memory,
    InvalidState,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum BungeeState {
    Created,
    Prerolled,
    GrainSpecified,
    GrainAnalysed,
    GrainSynthesised,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BungeeSampleRates {
    pub input_rate: c_float,
    pub output_rate: c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BungeeRequest {
    pub time_ratio: c_float,
    pub pitch_scale: c_float,
    pub window_size: usize,
    pub step_size: usize,
}

#[repr(C)]
pub struct BungeeStretcher {
    _private: [u8; 0],
}

extern "C" {
    // Initialization and cleanup
    pub fn bungee_init() -> BungeeError;
    pub fn bungee_cleanup();

    // Core stretcher functions
    pub fn bungee_create(rates: BungeeSampleRates, channels: c_int) -> *mut BungeeStretcher;
    pub fn bungee_destroy(stretcher: *mut BungeeStretcher);

    // Processing functions
    pub fn bungee_preroll(
        stretcher: *mut BungeeStretcher,
        request: *const BungeeRequest,
    ) -> BungeeError;

    pub fn bungee_specify_grain(
        stretcher: *mut BungeeStretcher,
        input_data: *const c_float,
        frame_count: usize,
    ) -> BungeeError;

    pub fn bungee_analyse_grain(stretcher: *mut BungeeStretcher) -> BungeeError;

    pub fn bungee_synthesise_grain(
        stretcher: *mut BungeeStretcher,
        output_data: *mut c_float,
        frame_count: usize,
    ) -> BungeeError;

    pub fn bungee_next(stretcher: *mut BungeeStretcher) -> BungeeError;

    // Query functions
    pub fn bungee_is_flushed(stretcher: *const BungeeStretcher) -> bool;
    pub fn bungee_max_input_frame_count(stretcher: *const BungeeStretcher) -> usize;

    // Debug and monitoring functions
    pub fn bungee_print_memory_stats();
    pub fn bungee_set_debug_log(log_file: *mut c_void);
}

// Safe Rust wrapper
pub struct Stretcher {
    inner: *mut BungeeStretcher,
}

unsafe impl Send for Stretcher {}
unsafe impl Sync for Stretcher {}

impl Stretcher {
    pub fn new(input_rate: f32, output_rate: f32, channels: i32) -> Option<Self> {
        let rates = BungeeSampleRates {
            input_rate,
            output_rate,
        };

        unsafe {
            let inner = bungee_create(rates, channels);
            if inner.is_null() {
                None
            } else {
                Some(Stretcher { inner })
            }
        }
    }

    pub fn preroll(&mut self, request: &BungeeRequest) -> Result<(), BungeeError> {
        unsafe {
            match bungee_preroll(self.inner, request) {
                BungeeError::Ok => Ok(()),
                err => Err(err),
            }
        }
    }

    pub fn specify_grain(&mut self, input_data: &[f32]) -> Result<(), BungeeError> {
        unsafe {
            match bungee_specify_grain(self.inner, input_data.as_ptr(), input_data.len()) {
                BungeeError::Ok => Ok(()),
                err => Err(err),
            }
        }
    }

    pub fn analyse_grain(&mut self) -> Result<(), BungeeError> {
        unsafe {
            match bungee_analyse_grain(self.inner) {
                BungeeError::Ok => Ok(()),
                err => Err(err),
            }
        }
    }

    pub fn synthesise_grain(&mut self, output_data: &mut [f32]) -> Result<(), BungeeError> {
        unsafe {
            match bungee_synthesise_grain(self.inner, output_data.as_mut_ptr(), output_data.len()) {
                BungeeError::Ok => Ok(()),
                err => Err(err),
            }
        }
    }

    pub fn next(&mut self) -> Result<(), BungeeError> {
        unsafe {
            match bungee_next(self.inner) {
                BungeeError::Ok => Ok(()),
                err => Err(err),
            }
        }
    }

    pub fn is_flushed(&self) -> bool {
        unsafe { bungee_is_flushed(self.inner) }
    }

    pub fn max_input_frame_count(&self) -> usize {
        unsafe { bungee_max_input_frame_count(self.inner) }
    }
}

impl Drop for Stretcher {
    fn drop(&mut self) {
        unsafe {
            bungee_destroy(self.inner);
        }
    }
}

// Initialize FFI on module load
pub fn init() -> Result<(), BungeeError> {
    unsafe {
        match bungee_init() {
            BungeeError::Ok => Ok(()),
            err => Err(err),
        }
    }
}

// Cleanup FFI on module unload
pub fn cleanup() {
    unsafe {
        bungee_cleanup();
    }
}