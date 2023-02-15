use crate::api::AuthError;
use serde::{Serialize, Deserialize};
use serde_json;
use super::FailResponse;

pub async fn sign_in_email(email: &str, password: &str, return_secure_token: bool) 
    -> Result<Response, AuthError> {

     let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        "AIzaSyBvAE59iedRLnTKZYR1XRLw_4ozM8sx80k",
    );

    let request = serde_json::json!({
        "email": email,
        "password": password,
        "return_secure_token": return_secure_token
    });



    let client = awc::Client::new();
    let mut resp = client.post(&url)
        .insert_header(("Content-Type", "application/json"))
        .send_json(&request)
        .await?;

    if resp.status() != 200 {
        let error = resp.json::<FailResponse>().await?.error;
        return Err(AuthError::SignIn(error.message));
    }
    let body = resp.json::<Response>().await?;

    Ok(body)

}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignInPayload<'a> {
    email: &'a str,
    password: &'a str,
    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub kind: String,
    pub local_id: String,
    pub email: String,
    pub display_name: String,
    pub id_token: String,
    pub registered: bool,
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>,
}
