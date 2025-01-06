//! Safe Rust wrapper for the Bungee audio time stretching library
//! This module provides a safe interface to the Bungee library's functionality.

mod ffi;
mod error;
mod monitoring;
mod recursion;

use std::sync::Once;
use error::BungeeError;
use monitoring::PerformanceGuard;
use recursion::RecursionGuard;

static INIT: Once = Once::new();

/// A time-stretching request with parameters for audio processing.
#[derive(Debug, Clone)]
pub struct Request {
    /// Time ratio for stretching (1.0 = no change)
    pub time_ratio: f32,
    /// Pitch scale factor (1.0 = no change)
    pub pitch_scale: f32,
    /// Window size for analysis
    pub window_size: usize,
    /// Step size between windows
    pub step_size: usize,
}

impl From<Request> for ffi::BungeeRequest {
    fn from(req: Request) -> Self {
        ffi::BungeeRequest {
            time_ratio: req.time_ratio,
            pitch_scale: req.pitch_scale,
            window_size: req.window_size,
            step_size: req.step_size,
        }
    }
}

/// A time-stretching engine for audio processing.
pub struct Stretcher {
    inner: ffi::Stretcher,
}

impl Stretcher {
    /// Creates a new stretcher instance.
    ///
    /// # Arguments
    /// * `input_rate` - Input sample rate in Hz
    /// * `output_rate` - Output sample rate in Hz
    /// * `channels` - Number of audio channels
    ///
    /// # Returns
    /// * `Ok(Stretcher)` on success
    /// * `Err(BungeeError)` if creation fails
    pub fn new(input_rate: f32, output_rate: f32, channels: i32) -> Result<Self, BungeeError> {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::new");

        // Initialize FFI on first use
        INIT.call_once(|| {
            if let Err(e) = ffi::init() {
                panic!("Failed to initialize Bungee: {:?}", e);
            }
        });

        ffi::Stretcher::new(input_rate, output_rate, channels)
            .map(|inner| Stretcher { inner })
            .ok_or(BungeeError::CreationFailed)
    }

    /// Prepares the stretcher for processing with the given parameters.
    ///
    /// # Arguments
    /// * `request` - Processing parameters
    pub fn preroll(&mut self, request: &Request) -> Result<(), BungeeError> {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::preroll");
        
        let ffi_request = ffi::BungeeRequest::from(request.clone());
        Ok(self.inner.preroll(&ffi_request)?)
    }

    /// Specifies input audio data for the next grain.
    ///
    /// # Arguments
    /// * `input_data` - Input audio samples
    pub fn specify_grain(&mut self, input_data: &[f32]) -> Result<(), BungeeError> {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::specify_grain");
        
        Ok(self.inner.specify_grain(input_data)?)
    }

    /// Analyzes the current grain.
    pub fn analyse_grain(&mut self) -> Result<(), BungeeError> {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::analyse_grain");
        
        Ok(self.inner.analyse_grain()?)
    }

    /// Synthesizes the current grain into output audio data.
    ///
    /// # Arguments
    /// * `output_data` - Buffer to receive output audio samples
    pub fn synthesise_grain(&mut self, output_data: &mut [f32]) -> Result<(), BungeeError> {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::synthesise_grain");
        
        Ok(self.inner.synthesise_grain(output_data)?)
    }

    /// Advances to the next grain.
    pub fn next(&mut self) -> Result<(), BungeeError> {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::next");
        
        Ok(self.inner.next()?)
    }

    /// Returns true if all input has been processed.
    pub fn is_flushed(&self) -> bool {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::is_flushed");
        
        self.inner.is_flushed()
    }

    /// Returns the maximum number of input frames that can be processed at once.
    pub fn max_input_frame_count(&self) -> usize {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::max_input_frame_count");
        
        self.inner.max_input_frame_count()
    }
}

impl Drop for Stretcher {
    fn drop(&mut self) {
        let _guard = RecursionGuard::new();
        let _perf = PerformanceGuard::new("Stretcher::drop");
        
        // Cleanup FFI when last stretcher is dropped
        ffi::cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stretcher_creation() {
        let stretcher = Stretcher::new(44100.0, 44100.0, 2);
        assert!(stretcher.is_ok());
    }

    #[test]
    fn test_invalid_channels() {
        let stretcher = Stretcher::new(44100.0, 44100.0, 0);
        assert!(stretcher.is_err());
    }

    #[test]
    fn test_processing_chain() {
        let mut stretcher = Stretcher::new(44100.0, 44100.0, 2).unwrap();
        
        let request = Request {
            time_ratio: 1.0,
            pitch_scale: 1.0,
            window_size: 1024,
            step_size: 512,
        };

        assert!(stretcher.preroll(&request).is_ok());
        
        let input = vec![0.0f32; 1024];
        assert!(stretcher.specify_grain(&input).is_ok());
        assert!(stretcher.analyse_grain().is_ok());
        
        let mut output = vec![0.0f32; 1024];
        assert!(stretcher.synthesise_grain(&mut output).is_ok());
        assert!(stretcher.next().is_ok());
    }

    #[test]
    fn test_max_frame_count() {
        let stretcher = Stretcher::new(44100.0, 44100.0, 2).unwrap();
        assert!(stretcher.max_input_frame_count() > 0);
    }
}
