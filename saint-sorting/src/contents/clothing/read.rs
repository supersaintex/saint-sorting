use crate::*;

pub async fn clothing_read(
    params: web::Form<FormParamsDbRead>,
    session: Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    read_firestore(session, params, tmpl).await
}
