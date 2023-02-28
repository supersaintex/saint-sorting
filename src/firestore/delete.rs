use crate::*;

pub async fn delete_firestore(
    session: Session,
    document_id: String,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(i) => i.to_string(),
    };

    let context = Context::new();
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let path = user_id + &String::from("/") + &document_id;
    let _result = documents::delete(&auth, &path, true);

    let view = tmpl
        .render("db_top.html", &context)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete {
    document_id: String,
}
