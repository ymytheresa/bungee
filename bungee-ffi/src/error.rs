use thiserror::Error;

#[derive(Error, Debug)]
pub enum BungeeError {
    #[error("Null pointer encountered")]
    NullPointer,
    
    #[error("Invalid parameter")]
    InvalidParam,
    
    #[error("Memory allocation failed")]
    Memory,
} 