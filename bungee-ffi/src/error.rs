use thiserror::Error;

#[derive(Error, Debug)]
pub enum BungeeError {
    #[error("Null pointer encountered")]
    NullPointer,
    
    #[error("Invalid parameter")]
    InvalidParam,
    
    #[error("Memory allocation failed")]
    Memory,

    #[error("Invalid state")]
    InvalidState,

    #[error("Buffer too small")]
    BufferTooSmall,
}

impl From<u32> for BungeeError {
    fn from(error: u32) -> Self {
        match error {
            1 => BungeeError::NullPointer,    // BUNGEE_NULL_POINTER
            2 => BungeeError::InvalidParam,    // BUNGEE_INVALID_PARAM
            3 => BungeeError::Memory,          // BUNGEE_MEMORY
            4 => BungeeError::InvalidState,    // BUNGEE_INVALID_STATE
            5 => BungeeError::BufferTooSmall,  // BUNGEE_BUFFER_TOO_SMALL
            _ => BungeeError::InvalidState,    // Default case
        }
    }
} 