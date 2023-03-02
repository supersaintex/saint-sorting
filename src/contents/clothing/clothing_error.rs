use thiserror::Error;

use crate::firestore::firestore_error;

#[derive(Error, Debug, Clone)]
pub enum ReadListError {
    #[error("{0}")]
    Firebase(String),

    #[error("{0}")]
    SessionGet(String),
    #[error("{0}")]
    ActixWeb(String),
}

impl std::convert::From<firestore_db_and_auth::errors::FirebaseError> for ReadListError {
    fn from(err: firestore_db_and_auth::errors::FirebaseError) -> Self {
        ReadListError::Firebase(err.to_string())
    }
}

impl std::convert::From<firestore_error::FireStoreError> for ReadListError {
    fn from(err: firestore_error::FireStoreError) -> Self {
        ReadListError::Firebase(err.to_string())
    }
}

impl actix_web::error::ResponseError for ReadListError {}

impl std::convert::From<actix_session::SessionGetError> for ReadListError {
    fn from(err: actix_session::SessionGetError) -> Self {
        ReadListError::SessionGet(err.to_string())
    }
}

impl std::convert::From<actix_web::Error> for ReadListError {
    fn from(err: actix_web::Error) -> Self {
        ReadListError::ActixWeb(err.to_string())
    }
}
