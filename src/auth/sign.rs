use super::FailResponse;
use crate::auth::AuthError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

pub async fn sign_in_email(
    email: &str,
    password: &str,
    return_secure_token: bool,
) -> Result<SignInResponse, AuthError> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        "AIzaSyBvAE59iedRLnTKZYR1XRLw_4ozM8sx80k",
    );
    let request = serde_json::json!({
        "email": email,
        "password": password,
        "return_secure_token": return_secure_token,
    });

    let auth_result = auth(&request, &url).await;

    if let Err(AuthError::Auth(e)) = auth_result {
        return Err(AuthError::SignIn(e));
    }

    auth_result
}

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
        "return_secure_token": return_secure_token,
    });

    let auth_result = auth(&request, &url).await;

    if let Err(AuthError::Auth(e)) = auth_result {
        return Err(AuthError::SignUp(e));
    }

    auth_result
}

pub async fn auth<T: DeserializeOwned>(
    request: &serde_json::Value,
    url: &str,
) -> Result<T, AuthError> {
    let client = awc::Client::new();
    let mut resp = client
        .post(url)
        .insert_header(("Content-Type", "application/json"))
        .send_json(&request)
        .await?;

    if resp.status() != 200 {
        let error = resp.json::<FailResponse>().await?.error;
        return Err(AuthError::Auth(error.message));
    }

    let body = resp.json::<T>().await?;

    Ok(body)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayLoad<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInResponse {
    pub kind: String,
    pub local_id: String,
    pub email: String,
    pub display_name: String,
    pub id_token: String,
    pub registered: bool,
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>,
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
