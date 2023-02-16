use crate::*;

pub async fn clothing_read(
    session: Session,
    params: web::Form<FormParamsDbRead>,
    // document_id: String,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, Error> {

    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(i) => i.to_string()
    };

    let context = Context::new();
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap(); 
    let doc_id  =  String::from(&params.document_id);

    let obj: DemoDTOClothing = documents::read(&auth, &user_id, doc_id).unwrap();
    println!("read start");
    println!("{}",obj.brand);
    println!("{}",obj.year);
    println!("{}",obj.month);
    println!("read end");

    let view = tmpl.render("clothing.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead{
    document_id: String
}

#[derive(Serialize, Deserialize)]
struct DemoDTOClothing {
    brand: String,
    year: u32,
    month: u32,
 }
