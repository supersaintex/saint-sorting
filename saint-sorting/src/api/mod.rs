pub mod sign_up;
pub mod sign_in;
pub mod auth_error;

pub use sign_in::{Response as SignInResponse};
pub use sign_up::{Response as SignUpResponse};
pub use auth_error::{Error as AuthError};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponse {
    error: FailResponseBody
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponseBody {
    message: String,
}
