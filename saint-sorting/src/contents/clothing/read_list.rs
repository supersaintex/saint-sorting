use crate::*;
// use firestore_db_and_auth::dto::Document;
// use firestore_db_and_auth::errors::FirebaseError;
use firestore_db_and_auth::{documents};


pub async fn clothing_read_list(
    session: Session,
    // params: web::Form<FormParamsDbRead>,
    // document_id: String,
    tmpl: web::Data<Tera>,
    ) 
    // -> actix_web::Result<HttpResponse, Error>
    -> actix_web::Result<HttpResponse, super::clothing_error::MyError>
    // ->Result<(DTOClothing, Document), FirebaseError>
    {

    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(i) => i.to_string()
    };
    // let user_id = String::from("921e065a-24b9-468f-8df4-aea2b335e95c");

    
    let context = Context::new();
    
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap(); 

    let values: documents::List<DTOClothing, _> = documents::list(&auth, &user_id);
    for doc_result in values {
        // The document is wrapped in a Result<> because fetching new data could have failed
        let (doc, _metadata) = doc_result?;
        // let (doc, _metadata) = doc_result.unwrap();
        
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