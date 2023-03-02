use actix_session::Session;
use actix_web::{error, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tera::{Context, Tera};
use uuid::Uuid;

mod auth;

pub fn render(
    tmpl: web::Data<Tera>,
    context: &Context,
    html_name: &str,
) -> actix_web::Result<HttpResponse, Error> {
    let view = tmpl
        .render(html_name, context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    email: String,
    passwd: String,
}

pub async fn top(tmpl: web::Data<Tera>) -> actix_web::Result<HttpResponse, Error> {
    let context = Context::new();
    render(tmpl, &context, "top.html")
}

pub async fn top_signup(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();

    let new_email = String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    //match auth::sign_up::sign_up_email(&new_email, &new_passwd, false).await {
    match auth::sign::sign_up_email(&new_email, &new_passwd, false).await {
        Ok(_response) => println!("signup successed"),
        Err(err) => {
            println!("Error : {err}");
            context.insert("failure_message", "signup failed...");
        }
    }

    render(tmpl, &context, "top.html")
}

pub async fn top_signin(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();

    let email = String::from(&params.email);
    let passwd = String::from(&params.passwd);

    //match auth::sign_in::sign_in_email(&email, &passwd, false).await {
    match auth::sign::sign_in_email(&email, &passwd, false).await {
        Ok(_response) => println!("signin successed"),
        Err(err) => {
            println!("Error : {err}");
            context.insert("failure_message", "signin failed...");
            return render(tmpl, &context, "top.html");
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
    render(tmpl, &context, "user_home.html")
}

pub async fn home(tmpl: web::Data<Tera>) -> actix_web::Result<HttpResponse, Error> {
    let context = Context::new();
    render(tmpl, &context, "home.html")
}

