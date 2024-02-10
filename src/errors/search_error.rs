use core::fmt;
use std::error::Error;

#[derive(Debug)]
enum SearchError {
    InvalidRegexPattern(String),
    IoError(std::io::Error),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::InvalidRegexPattern(pattern) => {
                write!(f, "Invalid regex pattern {}", pattern)
            }
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
