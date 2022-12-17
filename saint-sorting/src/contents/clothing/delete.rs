use crate::*;

pub async fn clothing_delete(
    params: web::Form<FormParamsDbDelete>,
    session: Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    delete_firestore(session, params, tmpl).await
}