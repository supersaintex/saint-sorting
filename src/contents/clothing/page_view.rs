use crate::*;

pub async fn clothing(
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let mut context = Context::new();
    if session.get::<Uuid>("user_id")?.is_none() {
        //return Ok(HttpResponse::Unauthorized().finish());
        context.insert("failure_message", "authentication failed, back to top page");
        return saint_sorting::render(tmpl, &context, "top.html");
    }

    return clothing_read_list(session, tmpl).await;
}
