use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, error,
                cookie::Key, middleware, HttpMessage as _};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use firestore_db_and_auth::{documents, documents::List, Credentials, 
                            ServiceSession, errors::Result, errors::FirebaseError};

mod api;
mod auth_error;

// --------------------
// firestore test
// --------------------

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

// for write firestore_db
#[derive(Serialize, Deserialize)]
pub struct FormParams_db_test {
    document_id: String,
    a_string: String,
    an_int: u32,
    another_int: u32,
}

// fn write(session: &ServiceSession) -> Result<()> {
//     let obj = DemoDTO { a_string: "abcd".to_owned(), an_int: 14, another_int: 10 };

//     let result = documents::write(session, "ss", Some("service_test"), &obj, documents::WriteOptions::default())?;

//     println!("id: {}, created: {}, updated: {}", result.document_id, result.create_time.unwrap(), result.update_time.unwrap());
//     Ok(())
// }


async fn write_top(
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let context = Context::new();
    let view = tmpl.render("write_firestore.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

async fn write_firestore(
    params: web::Form<FormParams_db_test>,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, Error> {

    
    let context = Context::new();
    
    let new_doc_id  =  String::from(&params.document_id);
    let new_a_string = String::from(&params.a_string);
    // let new_an_int = u32::from(&params.an_int);
    let new_an_int = params.an_int;
    let new_another_int = params.another_int;

    let obj = DemoDTO { a_string: new_a_string, an_int: new_an_int, another_int: new_another_int};

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let result = documents::write(&auth, "ss", Some(new_doc_id), &obj, documents::WriteOptions::default());

    // println!("id: {}, created: {}, updated: {}", result.document_id, result.create_time.unwrap(), result.update_time.unwrap());
    
    let view = tmpl.render("write_firestore.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
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
    -> actix_web::Result<HttpResponse, Error> {

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

async fn home() -> impl Responder {
     "hello home!"
}

async fn clothing() -> impl Responder {
     "hello clothing!"
}
async fn book() -> impl Responder {
    "hello book!"
}

// ---------
//test write
// ---------

// fn main() -> std::io::Result<()>{
//     let cred = Credentials::from_file("firebase-service-account.json").unwrap();
//     let auth = ServiceSession::new(cred).unwrap();

//     write(&auth);

//     Ok(())
// }

// test delete
// fn main() -> std::io::Result<()>{
//     let cred = Credentials::from_file("firebase-service-account.json").unwrap();
//     let auth = ServiceSession::new(cred).unwrap();

//     documents::delete(&auth, "ss/service_test", true);

//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Generate a random secret key. Note that it is important to use a unique
    // secret key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let secret_key = Key::generate();

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
            // .expect("Read credentials file")
            // .download_jwkset()
            // .expect("Failed to download public keys");

    let auth = ServiceSession::new(cred).unwrap();
            // .expect("Create a service account session");

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
            web::scope("/app")
                .route("/top", web::get().to(top))
                .route("/home", web::get().to(home))
                .route("/clothing", web::get().to(clothing))
                .route("/book", web::get().to(book))
                .route("/writetop", web::get().to(write_top))
                .route("/top/signup", web::post().to(top_signup))
                .route("/top/signin", web::post().to(top_signin))
                .route("/writetop/firestoretest", web::post().to(write_firestore))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
