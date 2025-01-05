//! Error handling for the Bungee library wrapper

use std::fmt;

/// Errors that can occur during Bungee operations
#[derive(Debug)]
pub enum Error {
    /// Failed to initialize the Bungee stretcher
    InitializationError(String),
    
    /// Invalid parameters provided to a function
    InvalidParameters(String),
    
    /// Error during audio processing
    ProcessingError(String),
    
    /// Buffer allocation or management error
    BufferError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitializationError(msg) => write!(f, "Failed to initialize Bungee: {}", msg),
            Error::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            Error::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            Error::BufferError(msg) => write!(f, "Buffer error: {}", msg),
        }
    }
}

/// Result type for Bungee operations
pub type Result<T> = std::result::Result<T, Error>; 