use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    API(String),

    #[error("{0}")]
    SignUp(String),

    #[error("{0}")]
    SignIn(String),

    // #[error("{0}")]
    // User(String),

    // #[error("{0}")]
    // Token(String),
}

impl std::convert::From<awc::error::SendRequestError> for Error {
    fn from(err: awc::error::SendRequestError) -> Self {
        Error::API(err.to_string())
    }
}

impl std::convert::From<awc::error::JsonPayloadError> for Error {
    fn from(err: awc::error::JsonPayloadError) -> Self {
        Error::API(err.to_string())
    }
}
