pub mod auth_error;
pub mod page_view;
pub mod sign;

pub use auth_error::Error as AuthError;

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
