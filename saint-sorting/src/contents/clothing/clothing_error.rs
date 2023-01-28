use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MyError {
    #[error("{0}")]
    FirebaseError(String),

    #[error("{0}")]
    ActixWebResponseError(String),

    #[error("{0}")]
    SessionGetError(String),
    
    #[error("{0}")]
    ActixWebError(String),
}

impl std::convert::From<firestore_db_and_auth::errors::FirebaseError> for MyError {
    fn from(err: firestore_db_and_auth::errors::FirebaseError) -> Self {
        MyError::FirebaseError(err.to_string())
    }
}


// Use default implementation for `error_response()` method
impl actix_web::error::ResponseError for MyError {}

impl std::convert::From<actix_session::SessionGetError> for MyError {
    fn from(err: actix_session::SessionGetError) -> Self {
        MyError::SessionGetError(err.to_string())
    }
}

impl std::convert::From<actix_web::Error> for MyError {
    fn from(err: actix_web::Error) -> Self {
        MyError::ActixWebError(err.to_string())
    }
}