use crate::firestore::{delete::delete_firestore, firestore_error};
use crate::*;

use super::read::clothing_read_list_render;

pub async fn clothing_delete(
    params: web::Form<FormParamsDbDelete>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let delete_doc_id: String = String::from(&params.document_id);

    let mut context = Context::new();
    match delete_firestore(&session, delete_doc_id).await {
        Ok(_) => (),
        Err(firestore_error::FireStoreError::Firebase(e)) => {
            context.insert("failure_message_delete", "deleting failed...");
            println!("firebase reading error: {e}");
            return saint_sorting::render(tmpl, &context, "clothing.html");
        }
        Err(firestore_error::FireStoreError::SessionGet(e)) => {
            context.insert("failure_message", "authentication failed...");
            println!("session_error: {e}");
            return saint_sorting::render(tmpl, &context, "top.html");
        }
        Err(firestore_error::FireStoreError::ActixWeb(e)) => {
            context.insert("failure_message", "server error...");
            println!("actixweb_error: {e}");
            return saint_sorting::render(tmpl, &context, "top.html");
        }
    }
    clothing_read_list_render(session, tmpl, &mut context).await
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete {
    document_id: String,
}
