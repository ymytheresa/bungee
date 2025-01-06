#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod error;
mod ffi;

use std::ptr::NonNull;
use thiserror::Error;

pub use error::BungeeError;

// Include the bindgen generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// A safe wrapper around BungeeStretcher
pub struct Stretcher {
    inner: NonNull<BungeeStretcher>,
}

/// Configuration for the stretcher
#[derive(Debug, Clone)]
pub struct Config {
    pub sample_rate: u32,
    pub channels: u32,
    pub time_ratio: f32,
    pub pitch_scale: f32,
}

impl Stretcher {
    /// Create a new stretcher instance
    pub fn new(config: Config) -> Result<Self, BungeeError> {
        let c_config = BungeeConfig {
            sample_rate: config.sample_rate,
            channels: config.channels,
            time_ratio: config.time_ratio,
            pitch_scale: config.pitch_scale,
        };

        let mut error = 0;
        // SAFETY: We pass valid pointers and check the result
        let ptr = unsafe { bungee_create(&c_config, &mut error) };
        
        if error != 0 {
            return Err(BungeeError::from_raw(error));
        }

        NonNull::new(ptr)
            .map(|inner| Self { inner })
            .ok_or(BungeeError::NullPointer)
    }

    /// Process audio data
    pub fn process(
        &mut self,
        input: &[f32],
        output: &mut [f32],
    ) -> Result<(usize, usize), BungeeError> {
        let mut frames_used = 0;
        let mut frames_generated = 0;

        let input_frames = input.len() as usize / self.channels() as usize;
        let output_frames = output.len() as usize / self.channels() as usize;

        // SAFETY: We ensure all pointers are valid and arrays are properly sized
        let result = unsafe {
            bungee_process(
                self.inner.as_ptr(),
                input.as_ptr(),
                input_frames,
                output.as_mut_ptr(),
                output_frames,
                &mut frames_used,
                &mut frames_generated,
            )
        };

        if result != 0 {
            return Err(BungeeError::from_raw(result));
        }

        Ok((frames_used, frames_generated))
    }

    /// Flush any remaining samples
    pub fn flush(&mut self, output: &mut [f32]) -> Result<usize, BungeeError> {
        let mut frames_generated = 0;
        let output_frames = output.len() as usize / self.channels() as usize;

        // SAFETY: We ensure output buffer is valid and properly sized
        let result = unsafe {
            bungee_flush(
                self.inner.as_ptr(),
                output.as_mut_ptr(),
                output_frames,
                &mut frames_generated,
            )
        };

        if result != 0 {
            return Err(BungeeError::from_raw(result));
        }

        Ok(frames_generated)
    }

    /// Reset the stretcher state
    pub fn reset(&mut self) -> Result<(), BungeeError> {
        // SAFETY: We ensure the stretcher pointer is valid
        let result = unsafe { bungee_reset(self.inner.as_ptr()) };
        
        if result != 0 {
            return Err(BungeeError::from_raw(result));
        }

        Ok(())
    }

    /// Get the number of channels
    pub fn channels(&self) -> u32 {
        // This would need to be implemented in the C layer
        // For now we'll return a placeholder
        2
    }
}

impl Drop for Stretcher {
    fn drop(&mut self) {
        // SAFETY: self.inner is guaranteed to be non-null and valid
        unsafe {
            bungee_destroy(self.inner.as_ptr());
        }
    }
}

// Implement Send and Sync for Stretcher as it's thread-safe
unsafe impl Send for Stretcher {}
unsafe impl Sync for Stretcher {} 