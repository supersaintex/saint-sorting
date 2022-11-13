// use crate::{error::Error};
use serde::{Serialize, Deserialize};
use super::FailResponse;

pub async fn sign_up_email(&self, email: &str, password: &str, return_secure_token: bool) -> Result<Response, Error> {

    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        self.api_key,
    );

    let client = reqwest::Client::new();
    
    let resp = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&SignUpPayload {
            email,
            password,
            return_secure_token
        })
        .send()
        .await?;

    if resp.status() != 200 {
        let error = resp.json::<FailResponse>().await?.error;
        return Err(Error::SignUp(error.message));
    }
    let body = resp.json::<Response>().await?;
    Ok(body)
}
