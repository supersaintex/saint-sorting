use actix_web::{web::{self, Data}, App, HttpServer, Responder, HttpResponse, Error, error};
use tera::{Tera, Context};

pub fn render(
    tmpl: web::Data<Tera>, context: &Context, html_name: &str)
    -> actix_web::Result<HttpResponse, Error> {

     let view = tmpl.render(html_name, &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}
