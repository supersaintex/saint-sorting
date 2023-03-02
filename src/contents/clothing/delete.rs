use crate::*;
use crate::firestore::firestore_error;

pub async fn clothing_delete(
    params: web::Form<FormParamsDbDelete>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, firestore_error::FireStoreError> {
    let delete_doc_id: String = String::from(&params.document_id);

    match delete_firestore(session, delete_doc_id).await {
        Ok(_) => (),
        Err(error) => return Err(error),
    }
    let context = Context::new();
    let view = tmpl
        .render("clothing.html", &context)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))

}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete {
    document_id: String,
}
