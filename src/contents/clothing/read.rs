use crate::{
    firestore::{
        firestore_error,
        read::{read_firestore, read_list_firestore},
    },
    *,
};
use firestore_db_and_auth::documents;

pub async fn clothing_read(
    session: Session,
    params: web::Form<FormParamsDbRead>,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let document_id = String::from(&params.document_id);

    let mut context = Context::new();

    let read_result: DTOClothing = match read_firestore(session, &auth, &document_id).await {
        Err(firestore_error::FireStoreError::Firebase(e)) => {
            context.insert("failure_message", "reading failed...");
            println!("firebase reading error: {e}");
            return saint_sorting::render(tmpl, &context, "clothing.html");
        }
        Err(firestore_error::FireStoreError::SessionGet(e)) => {
            context.insert("failure_message", "authentication failed...");
            println!("session_error: {e}");
            return saint_sorting::render(tmpl, &context, "top.html");
        }
        Err(firestore_error::FireStoreError::ActixWeb(e)) => {
            context.insert("failure_message", "server error...");
            println!("actixweb_error: {e}");
            return saint_sorting::render(tmpl, &context, "top.html");
        }

        Ok(dto) => dto,
    };

    println!("read start");
    println!("{}", read_result.brand);
    println!("{}", read_result.year);
    println!("{}", read_result.month);
    println!("{}", read_result.season);
    println!("{}", read_result.shop);
    println!("{}", read_result.category);
    println!("read end");

    let view = tmpl
        .render("clothing.html", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

pub async fn clothing_read_list(
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    //ServiceSession reference is included in documents::List, so get auth outside of
    //read_list_firestore function.
    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let mut context = Context::new();

    let read_list_result: documents::List<DTOClothing, _> =
        match read_list_firestore(session, &auth).await {
            Err(_) => {
                context.insert("failure_message", "reading failed...");
                return saint_sorting::render(tmpl, &context, "clothing.html");
            }
            Ok(dto_list) => dto_list,
        };

    for doc_result in read_list_result {
        let (doc, _metadata) = match doc_result {
            Err(_e) => {
                println!("cannot read doc");
                continue;
            }
            Ok(r) => r,
        };
        println!("{doc:?}");
    }

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
    season: String,
    shop: String,
    category: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}
