use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ReadListError {
    #[error("{0}")]
    FirebaseError(String),

    #[error("{0}")]
    SessionGetError(String),
    #[error("{0}")]
    ActixWebError(String),
}

impl std::convert::From<firestore_db_and_auth::errors::FirebaseError> for ReadListError {
    fn from(err: firestore_db_and_auth::errors::FirebaseError) -> Self {
        ReadListError::FirebaseError(err.to_string())
    }
}

impl actix_web::error::ResponseError for ReadListError {}

impl std::convert::From<actix_session::SessionGetError> for ReadListError {
    fn from(err: actix_session::SessionGetError) -> Self {
        ReadListError::SessionGetError(err.to_string())
    }
}

impl std::convert::From<actix_web::Error> for ReadListError {
    fn from(err: actix_web::Error) -> Self {
        ReadListError::ActixWebError(err.to_string())
    }
}
