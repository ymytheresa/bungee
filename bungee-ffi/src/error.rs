use thiserror::Error;

#[derive(Error, Debug)]
pub enum BungeeError {
    #[error("Null pointer encountered")]
    NullPointer,
    
    #[error("Invalid parameter")]
    InvalidParam,
    
    #[error("Memory allocation failed")]
    Memory,
    
    #[error("Processing error")]
    Processing,
    
    #[error("Unknown error: {0}")]
    Unknown(i32),
}

impl BungeeError {
    pub(crate) fn from_raw(error: i32) -> Self {
        match error {
            -1 => BungeeError::NullPointer,
            -2 => BungeeError::InvalidParam,
            -3 => BungeeError::Memory,
            -4 => BungeeError::Processing,
            other => BungeeError::Unknown(other),
        }
    }
} 