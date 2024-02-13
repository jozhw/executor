use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    CommandError(String, std::process::ExitStatus),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::IoError(err) => write!(f, "IO Error: {}", err),
            ExecutionError::Utf8Error(err) => write!(f, "UTF-8 Error: {}", err),
            ExecutionError::CommandError(message, status) => {
                write!(f, "Command Error: {}. Exit status: {:?}", message, status)
            }
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ExecutionError::IoError(err) => Some(err),
            ExecutionError::Utf8Error(err) => Some(err),
            ExecutionError::CommandError(_, _) => None,
        }
    }
}
