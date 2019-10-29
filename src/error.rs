use derive_error::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error(no_from, non_std)]
    Other {
        reason: String,
    },
    #[error(no_from, non_std)]
    ParseError,
    #[error(no_from, non_std)]
    InvalidArgumens {
        reason: String,
    },
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    RegexError(regex::Error),
    AddrError(addr::Error),
}
