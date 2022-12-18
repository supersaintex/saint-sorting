use crate::*;

pub async fn clothing(
    session: Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    if session.get::<Uuid>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    
    let context = Context::new();
    let view = tmpl.render("clothing.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}