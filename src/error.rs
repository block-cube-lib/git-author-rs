//! A module that defined application errors.

use thiserror::Error;

/// Application error
#[derive(Debug, Error)]
pub enum Error {
    /// Error returned when using "addr".
    #[error("Addr error: {0}")]
    AddrError(#[from] addr::Error),

    /// Command execution error.
    #[error("Command error: {0}")]
    CommandError(String),

    /// Error returned when using std::string::from_utf8.
    #[error("From utf8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// The argument passed when executiong the command is invalid.
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    /// Error returned when using std::io.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
