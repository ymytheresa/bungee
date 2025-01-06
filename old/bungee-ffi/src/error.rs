#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BungeeError {
    CreationFailed,
    NullPointer,
    InvalidParam,
    Memory,
    InvalidState,
    MaxRecursionDepth,
}

impl From<crate::ffi::BungeeError> for BungeeError {
    fn from(err: crate::ffi::BungeeError) -> Self {
        match err {
            crate::ffi::BungeeError::NullPointer => BungeeError::NullPointer,
            crate::ffi::BungeeError::InvalidParam => BungeeError::InvalidParam,
            crate::ffi::BungeeError::Memory => BungeeError::Memory,
            crate::ffi::BungeeError::InvalidState => BungeeError::InvalidState,
            crate::ffi::BungeeError::Ok => unreachable!(),
        }
    }
}

impl std::fmt::Display for BungeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BungeeError::CreationFailed => write!(f, "Failed to create stretcher instance"),
            BungeeError::NullPointer => write!(f, "Null pointer encountered"),
            BungeeError::InvalidParam => write!(f, "Invalid parameter provided"),
            BungeeError::Memory => write!(f, "Memory allocation failed"),
            BungeeError::InvalidState => write!(f, "Invalid state for operation"),
            BungeeError::MaxRecursionDepth => write!(f, "Maximum recursion depth exceeded"),
        }
    }
}

impl std::error::Error for BungeeError {} 