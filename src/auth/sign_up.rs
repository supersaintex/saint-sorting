use super::FailResponse;
use crate::auth::AuthError;
use serde::{Deserialize, Serialize};
use serde_json;

pub async fn sign_up_email(
    email: &str,
    password: &str,
    return_secure_token: bool,
) -> Result<SignUpResponse, AuthError> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        "AIzaSyBvAE59iedRLnTKZYR1XRLw_4ozM8sx80k",
    );

    let request = serde_json::json!({
        "email": email,
        "password": password,
        "return_secure_token": return_secure_token
    });

    let client = awc::Client::new();
    let mut resp = client
        .post(&url)
        .insert_header(("Content-Type", "application/json"))
        .send_json(&request)
        .await?;

    if resp.status() != 200 {
        let error = resp.json::<FailResponse>().await?.error;
        return Err(AuthError::SignUp(error.message));
    }

    let body = resp.json::<SignUpResponse>().await?;

    Ok(body)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignUpPayload<'a> {
    email: &'a str,
    password: &'a str,
    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUpResponse {
    pub id_token: String,
    pub email: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub local_id: String,
}
