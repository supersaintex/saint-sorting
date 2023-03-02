use crate::*;

pub async fn clothing_read(
    session: Session,
    params: web::Form<FormParamsDbRead>,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let document_id = String::from(&params.document_id);


    let read_result: DemoDTOClothing = match read_firestore(session, &auth, &document_id).await {
        Err(e) => return Err(e.into()),
        Ok(dto) => dto
    };

    println!("read start");
    println!("{}", read_result.brand);
    println!("{}", read_result.year);
    println!("{}", read_result.month);
    println!("read end");


    let context = Context::new();
    let view = tmpl
        .render("clothing.html", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}

#[derive(Serialize, Deserialize)]
struct DemoDTOClothing {
    brand: String,
    year: u32,
    month: u32,
}
