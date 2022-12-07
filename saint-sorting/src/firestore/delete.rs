use crate::*;

pub async fn delete_firestore(
    params: web::Form<FormParamsDbDelete>,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, Error> {

    
    let context = Context::new();
    
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();
   
    //path to document
    let path = String::from("ss/") + &params.document_id;
    let _result = documents::delete(&auth, &path, true);

    let view = tmpl.render("db_top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

// for delte from firestore_db
#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete{
    document_id: String
}