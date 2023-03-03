use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key,
    error,
    middleware::Logger,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
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
        delete::clothing_delete,
        page_view::clothing,
        read::{clothing_read, clothing_read_list},
        write::clothing_write,
    },
};

use firestore::{
    db_top::db_top, delete::delete_firestore, read::read_firestore, write::write_firestore,
};

use saint_sorting::{home, top};

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
