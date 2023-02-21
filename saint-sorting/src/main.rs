use actix_session::{
    storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{web::{self, Data}, App, HttpServer, Responder, HttpResponse, Error, error, cookie::Key, middleware::Logger};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use firestore_db_and_auth::{documents, Credentials, ServiceSession};

use uuid::Uuid;
use serde_json::json;


mod api;
mod firestore;
mod contents;

use firestore::{db_top::db_top, write::write_firestore,
                delete::delete_firestore, read::read_firestore,};

use contents::{clothing::{page_view::clothing, write::clothing_write,
                         read::clothing_read, delete::clothing_delete,
                         read_list::clothing_read_list}, book::page_view::book};

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
    saint_sorting::render(tmpl, &context, "top.html")
}

async fn top_signup(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let mut context = Context::new();

    let new_email  =  String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    match api::sign_up::sign_up_email(&new_email, &new_passwd, false).await {
        Ok(_response) => println!("signup successed"),
        Err(err) => {
            println!("Error : {}", err);
            context.insert("failure_message", "signup failed...");
        }

    }

    saint_sorting::render(tmpl, &context, "top.html")
}

async fn top_signin(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
    session: Session)
    -> actix_web::Result<HttpResponse, Error> {

    let mut context = Context::new();

    let email  =  String::from(&params.email);
    let passwd = String::from(&params.passwd);

    match api::sign_in::sign_in_email(&email, &passwd, false).await {
        Ok(_response) => println!("signin successed"),
        Err(err) => {
            println!("Error : {}", err);
            //return Ok(HttpResponse::Unauthorized().finish());
            context.insert("failure_message", "signin failed...");
            return saint_sorting::render(tmpl, &context, "top.html")
        },
    }


    let _json = match session.get::<Uuid>("user_id")? {
        Some(user_id) => {
            json!({ "user_id": &user_id });
            println!("Your_UserId_Is:{}", user_id);
            context.insert("UserId", &user_id);
        }
        None => {
            let user_id = Uuid::new_v4();
            session.insert("user_id", &user_id)?;
            json!({"user_id": &user_id });
            context.insert("UserId", &user_id);
        }
    };

    context.insert("name", &email);
    saint_sorting::render(tmpl, &context, "user_home.html")
}

async fn home() -> impl Responder {
    "hello home!"
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
            .app_data(Data::new(tera))
            .service(
                web::scope("/app")
                .route("/top", web::get().to(top))
                .route("/home", web::get().to(home))
                .route("/dbtop", web::get().to(db_top))
                .route("/top/signup", web::post().to(top_signup))
                .route("/top/signin", web::post().to(top_signin))
                .route("/book", web::get().to(book))
                .route("/clothing", web::get().to(clothing))
                .route("/clothing/write", web::post().to(clothing_write))
                .route("/clothing/read", web::post().to(clothing_read))
                .route("/clothing/read_list", web::get().to(clothing_read_list))
                .route("/clothing/delete", web::post().to(clothing_delete))
                // .route("/dbtop/writetest", web::post().to(write_firestore))
                .route("/dbtop/deletetest", web::post().to(delete_firestore))
                .route("/dbtop/readtest", web::post().to(read_firestore))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
