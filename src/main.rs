use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key,
    error,
    middleware::Logger,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Responder,
};
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tera::{Context, Tera};
use uuid::Uuid;

mod api;
mod contents;
mod firestore;

use contents::{
    book::page_view::book,
    clothing::{
        delete::clothing_delete,
        page_view::clothing,
        read::{clothing_read, clothing_read_list},
        write::clothing_write,
    },
};

// for sign up and in
#[derive(Serialize, Deserialize)]
pub struct FormParams {
    email: String,
    passwd: String,
}

async fn top(tmpl: web::Data<Tera>) -> actix_web::Result<HttpResponse, Error> {
    let context = Context::new();
    saint_sorting::render(tmpl, &context, "top.html")
}

async fn top_signup(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();

    let new_email = String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    match api::sign_up::sign_up_email(&new_email, &new_passwd, false).await {
        Ok(_response) => println!("signup successed"),
        Err(err) => {
            println!("Error : {err}");
            context.insert("failure_message", "signup failed...");
        }
    }

    saint_sorting::render(tmpl, &context, "top.html")
}

async fn top_signin(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();

    let email = String::from(&params.email);
    let passwd = String::from(&params.passwd);

    match api::sign_in::sign_in_email(&email, &passwd, false).await {
        Ok(_response) => println!("signin successed"),
        Err(err) => {
            println!("Error : {err}");
            //return Ok(HttpResponse::Unauthorized().finish());
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
                println!("parsing error(s): {e}");
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
            .service(actix_files::Files::new("/app/css", "templates/css/").show_files_listing())
            .service(
                web::scope("/app")
                    .route("/top", web::get().to(top))
                    .route("/home", web::get().to(home))
                    .route("/top/signup", web::post().to(top_signup))
                    .route("/top/signin", web::post().to(top_signin))
                    .route("/book", web::get().to(book))
                    .route("/clothing", web::get().to(clothing))
                    .route("/clothing/write", web::post().to(clothing_write))
                    .route("/clothing/read", web::post().to(clothing_read))
                    .route("/clothing/read_list", web::get().to(clothing_read_list))
                    .route("/clothing/delete", web::post().to(clothing_delete)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
