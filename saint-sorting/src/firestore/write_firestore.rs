use crate::*;

pub async fn write_firestore(
    params: web::Form<FormParamsDbWrite>,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, Error> {

    
    let context = Context::new();
    
    let new_doc_id  =  String::from(&params.document_id);
    let new_a_string = String::from(&params.a_string);
    let new_an_int = params.an_int;
    let new_another_int = params.another_int;

    let obj = DemoDTO { a_string: new_a_string, an_int: new_an_int, another_int: new_another_int};

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let _result = documents::write(&auth, "ss", Some(new_doc_id), &obj, documents::WriteOptions::default());

    // println!("id: {}, created: {}, updated: {}", result.document_id, result.create_time.unwrap(), result.update_time.unwrap());
    
    let view = tmpl.render("db_top.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

// for write firestore_db
#[derive(Serialize, Deserialize)]
pub struct FormParamsDbWrite {
    document_id: String,
    a_string: String,
    an_int: u32,
    another_int: u32,
}