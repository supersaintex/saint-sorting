use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::Logger,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use uuid::Uuid;

mod auth;
mod contents;
mod firestore;

use auth::page_view::{top_signin, top_signup};

use contents::{
    book::page_view::book,
    clothing::{
        delete::clothing_delete, page_view::clothing, read::clothing_read_list,
        write::clothing_write,
    },
};

use saint_sorting::{home, top};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("localhost+2-key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("localhost+2.pem")
        .unwrap();

    let redis_key = Key::generate();
    let redis_connection_string = "redis://127.0.0.1:6379";
    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

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
               /*  SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64])) */
                    /* .cookie_name("seint".to_owned()) */
                    /* .cookie_secure(false) */
                    /* .build(), */
                SessionMiddleware::new(
                    redis_store.clone(),
                    redis_key.clone()
                )
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
                    .route("/clothing/delete", web::post().to(clothing_delete)),
            )
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
