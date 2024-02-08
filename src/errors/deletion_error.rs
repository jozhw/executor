use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DeletionError {
    NotFound,
    PermissionDenied,
    Other(String),
}

impl fmt::Display for DeletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeletionError::NotFound => write!(f, "File not found"),
            DeletionError::PermissionDenied => write!(f, "Permission denied"),
            DeletionError::Other(message) => write!(f, "Error: {}", message),
        }
    }
}

impl Error for DeletionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeletionError::NotFound | DeletionError::PermissionDenied => None,
            DeletionError::Other(_) => Some(self),
        }
    }
}
