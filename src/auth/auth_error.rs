use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    Auth(String),

    #[error("{0}")]
    SignUp(String),

    #[error("{0}")]
    SignIn(String),
}

impl std::convert::From<awc::error::SendRequestError> for Error {
    fn from(err: awc::error::SendRequestError) -> Self {
        Error::Auth(err.to_string())
    }
}

impl std::convert::From<awc::error::JsonPayloadError> for Error {
    fn from(err: awc::error::JsonPayloadError) -> Self {
        Error::Auth(err.to_string())
    }
}
