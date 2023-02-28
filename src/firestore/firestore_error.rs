use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FireStoreError {
    #[error("{0}")]
    Firebase(String),

    #[error("{0}")]
    SessionGet(String),

    #[error("{0}")]
    ActixWeb(String),
}

impl std::convert::From<firestore_db_and_auth::errors::FirebaseError> for FireStoreError {
    fn from(err: firestore_db_and_auth::errors::FirebaseError) -> Self {
        FireStoreError::Firebase(err.to_string())
    }
}

// Use default implementation for `error_response()` method
impl actix_web::error::ResponseError for FireStoreError {}

impl std::convert::From<actix_session::SessionGetError> for FireStoreError {
    fn from(err: actix_session::SessionGetError) -> Self {
        FireStoreError::SessionGet(err.to_string())
    }
}

impl std::convert::From<actix_web::Error> for FireStoreError {
    fn from(err: actix_web::Error) -> Self {
        FireStoreError::ActixWeb(err.to_string())
    }
}
