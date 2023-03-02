use crate::{firestore::read::read_list_firestore, *};
use firestore_db_and_auth::documents;

pub async fn clothing_read_list(
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, super::clothing_error::ReadListError> {
    //ServiceSession reference is included in documents::List, so get auth outside of
    //read_list_firestore function.
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let read_list_result: documents::List<DTOClothing, _> =
        match read_list_firestore(session, &auth).await {
            Err(e) => return Err(e.into()),
            Ok(dto_list) => dto_list,
        };

    for doc_result in read_list_result {
        let (doc, _metadata) = doc_result?;
        println!("{doc:?}");
    }

    let context = Context::new();
    let view = tmpl
        .render("clothing.html", &context)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize, Debug)]
struct DTOClothing {
    brand: String,
    year: u32,
    month: u32,
}
