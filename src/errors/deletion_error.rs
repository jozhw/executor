use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DeletionError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    CommandError(String, std::process::ExitStatus),
}

impl fmt::Display for DeletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeletionError::IoError(err) => write!(f, "IO Error: {}", err),
            DeletionError::Utf8Error(err) => write!(f, "UTF-8 Error: {}", err),
            DeletionError::CommandError(message, status) => {
                write!(f, "Command Error: {}. Exit status: {:?}", message, status)
            }
        }
    }
}

impl Error for DeletionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeletionError::IoError(err) => Some(err),
            DeletionError::Utf8Error(err) => Some(err),
            DeletionError::CommandError(_, _) => None,
        }
    }
}
