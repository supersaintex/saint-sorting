use crate::*;
use firestore_db_and_auth::{documents};


pub async fn clothing_read_list(
    session: Session,
    tmpl: web::Data<Tera>,
    ) 
    -> actix_web::Result<HttpResponse, super::clothing_error::ReadListError>
    {

    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(i) => i.to_string()
    };
    let context = Context::new();
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap(); 

    let values: documents::List<DTOClothing, _> = documents::list(&auth, &user_id);
    for doc_result in values {
        // The document is wrapped in a Result<> because fetching new data could have failed
        let (doc, _metadata) = doc_result?;
        println!("{:?}", doc);
    }
    let view = tmpl.render("clothing.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead{
    document_id: String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct DTOClothing {
    brand: String,
    year: u32,
    month: u32,
 }
