use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, error};
use tera::{Tera, Context};

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error>  {

        let context = Context::new();
    let view = tmpl.render("index.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
    //"Hello world!"

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
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index))
                .route("/home", web::get().to(home))
                .route("/clothing", web::get().to(clothing))
                .route("/book", web::get().to(book)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
