use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, error,
                cookie::Key, middleware, HttpMessage as _};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

mod api;
mod auth_error;

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

async fn top_signup(
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
    match api::sign_up::sign_up_email(&new_email, &new_passwd, false).await {
        Ok(response) => println!("signup successed"),
        Err(err) => println!("Error : {}", err),
    }


    let view = tmpl.render("top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

async fn top_signin(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,)
    -> Result<HttpResponse, Error> {

    let context = Context::new();

    let new_email  =  String::from(&params.email);
    let new_passwd = String::from(&params.passwd);

    match api::sign_in::sign_in_email(&new_email, &new_passwd, false).await {
        Ok(response) => println!("sighin successed"),
        Err(err) => println!("Error : {}", err),
    }


    let view = tmpl.render("top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

// /// simple index handler with session
// async fn make_session(session: Session, req: HttpRequest) -> Result<&'static str> {
//     log::info!("{:?}", req);

//     // RequestSession trait is used for session access
//     let mut counter = 1;
//     if let Some(count) = session.get::<i32>("counter")? {
//         log::info!("SESSION value: {}", count);
//         counter = count + 1;
//         session.insert("counter", counter)?;
//     } else {
//         session.insert("counter", counter)?;
//     }

//     Ok("welcome!")
// }

// async fn login_v2(req: HttpRequest) -> HttpResponse {
//     Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

//     HttpResponse::Found()
//         .insert_header(("location", "/"))
//         .finish()
// }



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

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Generate a random secret key. Note that it is important to use a unique
    // secret key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let secret_key = Key::generate();

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
                .route("/top/signup", web::post().to(top_signup))
                .route("/top/signin", web::post().to(top_signin))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
