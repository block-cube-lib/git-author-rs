//! A module that defined application errors.

use crate::git::Author;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct InvalidEmailAddressError {
    address: String,
}

impl std::fmt::Display for InvalidEmailAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid email address error: {}", self.address)
    }
}

impl InvalidEmailAddressError {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn address(&self) -> &String {
        &self.address
    }
}

/// Application error
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid email address error: {0}")]
    InvalidEmailAddress(#[from] InvalidEmailAddressError),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(#[from] InvalidArguments),

    #[error("author field error: {0}")]
    AuthorField(#[from] AuthorFieldError),

    #[error("get error: {0}")]
    Get(#[from] GetError),

    #[error("set error: {0}")]
    Set(#[from] SetError),

    #[error("unset error: {0}")]
    Unset(#[from] UnsetError),

    #[error("replace error: {0}")]
    Replace(#[from] ReplaceError),
}

#[derive(Debug, Error)]
pub enum OutputError {
    /// Command execution error.
    #[error("Command execute error: {0}")]
    CommandExecute(#[from] CommandExecuteError),

    /// Error returned when using std::string::from_utf8.
    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    /// Error returned when using std::io.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[error("command execute error: {0}")]
pub struct CommandExecuteError(pub String);

#[derive(Debug, Error)]
#[error("Invalid arguments: {0}")]
pub struct InvalidArguments(pub String);

#[derive(Debug, Error)]
pub enum AuthorFieldError {
    #[error("name is `None`")]
    NameIsNone,
    #[error("eamil is `None`")]
    EmailIsNone,
    #[error("name and eamil are `None`")]
    NameAndEmailAreNone,
}

impl AuthorFieldError {
    pub fn new(author: &Author) -> Result<Self, InvalidArguments> {
        match (author.name(), author.email()) {
            (Some(_name), Some(_email)) => Err(InvalidArguments(
                "author has no None parameter.".to_string(),
            )),
            (None, None) => Ok(AuthorFieldError::NameAndEmailAreNone),
            (None, _) => Ok(AuthorFieldError::NameIsNone),
            (_, None) => Ok(AuthorFieldError::EmailIsNone),
        }
    }
}

#[derive(Debug, Error)]
#[error("auhtor has none field: {0}")]
pub struct AuthorHasNoneField(#[from] pub AuthorFieldError);

#[derive(Debug, Error)]
#[error("committer has none field: {0}")]
pub struct CommitterHasNoneField(#[from] pub AuthorFieldError);

#[derive(Debug, Error)]
pub enum ConditionTextError {
    #[error("author has None field: {0}")]
    AuthorHasNoneField(#[from] AuthorHasNoneField),

    #[error("committer has None field: {0}")]
    CommitterHasNoneField(#[from] CommitterHasNoneField),

    #[error("InvalidArguments: {0}")]
    InvalidArguments(#[from] InvalidArguments),
}

#[derive(Debug, Error)]
pub enum ProccessingContentError {
    #[error("author has None field: {0}")]
    AuthorHasNoneField(#[from] AuthorHasNoneField),

    #[error("committer has None field: {0}")]
    CommitterHasNoneField(#[from] CommitterHasNoneField),

    #[error("InvalidArguments: {0}")]
    InvalidArguments(#[from] InvalidArguments),
}

#[derive(Debug, Error)]
pub enum GetError {
    #[error("output error: {0}")]
    Output(#[from] OutputError),

    /// Error returned when parse email address.
    #[error("invalid email address error: {0}")]
    InvalidEmailAddress(#[from] InvalidEmailAddressError),
}

#[derive(Debug, Error)]
pub enum SetError {
    #[error("author field error: {0}")]
    AuthorField(#[from] AuthorFieldError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),
}

#[derive(Debug, Error)]
pub enum UnsetError {
    #[error("Command execute error: {0}")]
    CommandExecute(#[from] CommandExecuteError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),
}

#[derive(Debug, Error)]
pub enum ReplaceError {
    #[error("condition text error: {0}")]
    ConditionText(#[from] ConditionTextError),

    #[error("to proccessing content error: {0}")]
    ProccessingContent(#[from] ProccessingContentError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Command execute error: {0}")]
    CommandExecute(#[from] CommandExecuteError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),
}
