use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, error};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
// use firestore_db_and_auth::{documents::List, errors::Result, errors::FirebaseError};

use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{cookie::{self, Key}, middleware::Logger, HttpRequest, Result};
use uuid::Uuid;
use serde_json::json;


// -------------
// local modules
// -------------
mod api;
mod auth_error;
mod firestore;
mod session;

use firestore::{db_top::db_top, write::write_firestore, 
                delete::delete_firestore, read::read_firestore};


#[derive(Serialize, Deserialize)]
 struct DemoDTO {
    a_string: String,
    an_int: u32,
    another_int: u32,
 }
 #[derive(Serialize, Deserialize)]
 struct DemoPartialDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    a_string: Option<String>,
    an_int: u32,
 }


// for sign up and in
#[derive(Serialize, Deserialize)]
pub struct FormParams {
    email: String,
    passwd: String
}

async fn top(
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let context = Context::new();
    let view = tmpl.render("top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

async fn top_signup(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let context = Context::new();

    let new_email  =  String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    match api::sign_up::sign_up_email(&new_email, &new_passwd, false).await {
        Ok(_response) => println!("signup successed"),
        Err(err) => println!("Error : {}", err),
    }

    let view = tmpl.render("top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

async fn top_signin(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
    session: Session)
    -> actix_web::Result<HttpResponse, Error> {

    let mut context = Context::new();

    let new_email  =  String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    match api::sign_in::sign_in_email(&new_email, &new_passwd, false).await {
        Ok(_response) => println!("signin successed"),
        Err(err) => println!("Error : {}", err),
    }

    //access session data
    // if let Some(count) = session.get::<i32>("counter")? {
    //     session.insert("counter", count + 1)?;
    // } else {
    //     session.insert("counter", 1)?;
    // }

    let json = match session.get::<Uuid>("user_id")? {
        Some(user_id) => json!({ "user_id": &user_id }),
        None => {
            let user_id = Uuid::new_v4();
            session.insert("user_id", &user_id)?;

            json!({"user_id": &user_id })
        }
    };

    context.insert("name", &new_email);
    
    let view = tmpl.render("user_home.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

async fn home() -> impl Responder {
    "hello home!"
}

async fn clothing(
    session: Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    if session.get::<Uuid>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    
    let context = Context::new();
    let view = tmpl.render("clothing.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}


async fn book(
    session :Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    if session.get::<Uuid>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let context = Context::new();
    let view = tmpl.render("book.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    
    HttpServer::new(move || {

        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_name("seint".to_owned())
                    .cookie_secure(false)
                    .build(),
            )
            .data(tera)
            .service(
            web::scope("/app")
                .route("/top", web::get().to(top))
                .route("/home", web::get().to(home))
                .route("/dbtop", web::get().to(db_top))
                .route("/top/signup", web::post().to(top_signup))
                .route("/top/signin", web::post().to(top_signin))
                .route("/top/book", web::get().to(book))
                .route("/top/clothing", web::get().to(clothing))
                .route("/dbtop/writetest", web::post().to(write_firestore))
                .route("/dbtop/deletetest", web::post().to(delete_firestore))
                .route("/dbtop/readtest", web::post().to(read_firestore))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
