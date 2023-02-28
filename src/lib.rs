use actix_web::{error, web, Error, HttpResponse};
use tera::{Context, Tera};

pub fn render(
    tmpl: web::Data<Tera>,
    context: &Context,
    html_name: &str,
) -> actix_web::Result<HttpResponse, Error> {
    let view = tmpl
        .render(html_name, context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}
