use core::fmt;
use std::error::Error;

#[derive(Debug)]
enum SearchError {
    NotFound,
    PermissionDenied,
    IoError(std::io::Error),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::NotFound => write!(f, "File not found"),
            SearchError::PermissionDenied => write!(f, "Permission denied"),
            SearchError::IoError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl Error for SearchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SearchError::IoError(err) => Some(err),
            _ => None,
        }
    }
}
