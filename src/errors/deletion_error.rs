use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DeletionError {
    NotFound,
    PermissionDenied,
    Other(String),
    IoError(std::io::Error),
}

impl fmt::Display for DeletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeletionError::NotFound => write!(f, "File not found"),
            DeletionError::PermissionDenied => write!(f, "Permission denied"),
            DeletionError::Other(message) => write!(f, "Error: {}", message),
            DeletionError::IoError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl Error for DeletionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeletionError::NotFound | DeletionError::PermissionDenied => None,
            DeletionError::Other(_) => Some(self),
            DeletionError::IoError(err) => Some(err),
        }
    }
}
