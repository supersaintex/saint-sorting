use crate::*;

pub async fn clothing_delete(
    params: web::Form<FormParamsDbDelete>,
    session: Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let delete_doc_id: String  =  String::from(&params.document_id);

    delete_firestore(session, delete_doc_id, tmpl).await
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete{
    document_id: String
}