use crate::*;

pub async fn read_firestore(
    params: web::Form<FormParamsDbWrite>,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, Error> {

    
    let context = Context::new();
    
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap(); 
    
    let doc_id  =  String::from(&params.document_id);
    let obj : DemoDTO = documents::read(&auth, "ss", doc_id).unwrap();

    println!("{}",obj.an_int);
    println!("hello");

    let view = tmpl.render("db_top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}


#[derive(Serialize, Deserialize)]
pub struct FormParamsDbWrite{
    document_id: String
}