pub mod auth_error;
//pub mod sign_in;
//pub mod sign_up;
pub mod sign;
pub mod page_view;

pub use auth_error::Error as AuthError;
//pub use sign_in::SignInResponse;
pub use sign::SignInResponse;
//pub use sign_up::SignUpResponse;
pub use sign::SignUpResponse;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponse {
    error: FailResponseBody,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponseBody {
    message: String,
}
