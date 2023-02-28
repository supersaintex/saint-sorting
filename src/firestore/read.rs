use crate::*;

pub async fn read_firestore(
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

    let obj: MyDTO = documents::read(&auth, &user_id, document_id).unwrap();

    println!("read start");
    println!("{}", obj.a_string);
    println!("{}", obj.an_int);
    println!("{}", obj.another_int);
    println!("read end");

    let view = tmpl
        .render("db_top.html", &context)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}

#[derive(Serialize, Deserialize)]
struct MyDTO {
    a_string: String,
    an_int: i32,
    another_int: i32,
}
