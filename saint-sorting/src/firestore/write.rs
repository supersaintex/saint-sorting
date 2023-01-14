use crate::*;

pub async fn write_firestore<T>(
    session: Session,
    document_id: String,
    obj: &T,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, Error> 
    // -> Result<firestore_db_and_auth::documents::WriteResult, Error> 
    
where
    T: Serialize,
    {

    //ここを別の関数にする？
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        // None => return Ok(()),
        Some(i) => i.to_string()
    };

    let context = Context::new();
    
    // let new_doc_id  =  String::from(&params.document_id);
    // let new_a_string = String::from(&params.a_string);
    // let new_an_int = params.an_int;
    // let new_another_int = params.another_int;

    // let obj = DemoDTO { a_string: new_a_string, an_int: new_an_int, another_int: new_another_int};

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let _result = documents::write(&auth, &user_id, Some(document_id), &obj, documents::WriteOptions::default());
    // let result = documents::write(&auth, &user_id, Some(document_id), &obj, documents::WriteOptions::default())?;

    // Ok(result)
    
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