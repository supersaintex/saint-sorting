use crate::*;

use actix_session::Session;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::sign::AuthPayLoad;

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    email: String,
    password: String,
}

pub async fn top_signup(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();

    let new_email = String::from(&params.email);
    let new_password = String::from(&params.password);

    let authpayload = AuthPayLoad {
        email: &new_email,
        password: &new_password,
        return_secure_token: false,
    };

    match auth::sign::sign_up_email(&authpayload).await {
        Ok(_response) => println!("signup successed"),
        Err(err) => {
            println!("Error : {err}");
            context.insert("failure_message", "signup failed...");
        }
    }

    saint_sorting::render(tmpl, &context, "top.html")
}

pub async fn top_signin(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();

    let email = String::from(&params.email);
    let password = String::from(&params.password);

    let authpayload = AuthPayLoad {
        email: &email,
        password: &password,
        return_secure_token: false,
    };

    match auth::sign::sign_in_email(&authpayload).await {
        Ok(_response) => println!("signin successed"),
        Err(err) => {
            println!("Error : {err}");
            context.insert("failure_message", "signin failed...");
            return saint_sorting::render(tmpl, &context, "top.html");
        }
    }

    match session.get::<Uuid>("user_id")? {
        Some(user_id) => {
            json!({ "user_id": &user_id });
            println!("Your_UserId_Is:{user_id}");
            context.insert("UserId", &user_id);
        }
        None => {
            let user_id = Uuid::new_v4();
            session.insert("user_id", user_id)?;
            json!({ "user_id": &user_id });
            context.insert("UserId", &user_id);
        }
    };

    context.insert("name", &email);
    session.insert("email_address", email)?;
    saint_sorting::render(tmpl, &context, "user_home.html")
}
