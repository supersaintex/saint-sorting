use awc::{Client};
use crate::{auth_error::Error};
use serde::{Serialize, Deserialize};
use serde_json;
use super::FailResponse;


pub async fn sign_up_email(email: &str, password: &str, return_secure_token: bool) -> Result<(), Error> {

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
    let mut resp = client.post(&url)
        .insert_header(("Content-Type", "application/json"))
        .send_json(&request)
        .await?;

    if resp.status() != 200 {
        let error = resp.json::<FailResponse>().await?.error;
        return Err(Error::SignUp(error.message));
    }

    Ok(())




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
pub struct Response {
    pub id_token: String,
    pub email: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub local_id: String,
}
