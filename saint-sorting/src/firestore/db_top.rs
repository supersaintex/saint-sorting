use crate::*;

pub async fn db_top(
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let context = Context::new();
    let view = tmpl.render("db_top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}