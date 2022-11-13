use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, error};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
// use crate::{error::Error};

mod api;

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    email: String,
    passwd: String
}

async fn top(
    tmpl: web::Data<Tera>,)
    -> Result<HttpResponse, Error> {

    let context = Context::new();
    let view = tmpl.render("top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

async fn top_login(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,)
    -> Result<HttpResponse, Error> {

    let context = Context::new();

    let new_email  =  String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    // println!("{}", new_email);
    // println!("{}", new_passwd);
    // println!("{}", "hello");
    //
    //api::sign_up::sign_up_email(&new_email, &new_passwd, false).await.unwrap_or_else(|err| eprintln!("sign_up error : {}", err));
    match api::sign_up::sign_up_email(&new_email, &new_passwd, false).await {
        Ok(_) => println!("sign up successed"),
        Err(err) => println!("Error : {}", err),
    }


    let view = tmpl.render("top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}


async fn home() -> impl Responder {
    "hello home!"
}
async fn clothing() -> impl Responder {
    "hello clothing!"
}
async fn book() -> impl Responder {
    "hello book!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    // INPUT OUR API-KEY FROM 
    // let api_key: String = String::from("s6FqaFcRFd...njhB8cCjN7");

    // let auth = fireauth::FireAuth::new("AIzaSyBvAE59iedRLnTKZYR1XRLw_4ozM8sx80k");


    HttpServer::new(|| {

        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .data(tera)
            .service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/top.html`
                .route("/top", web::get().to(top))
                .route("/home", web::get().to(home))
                .route("/clothing", web::get().to(clothing))
                .route("/book", web::get().to(book))
                .route("/top/login", web::post().to(top_login))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
