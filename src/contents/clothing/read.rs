use crate::{
    firestore::{
        firestore_error,
        read::{read_firestore, read_list_firestore},
    },
    *,
};
use firestore_db_and_auth::documents;

fn split_name_to_id(name: &str) -> &str {
    let split_name: Vec<&str> = name.rsplit('/').collect();
    split_name[0]
}

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

    let json_read_result = serde_json::to_value(read_result).unwrap();
    //println!("{json_read_result}");
    context.insert("read_result", &json_read_result.to_string());

    saint_sorting::render(tmpl, &context, "clothing.html")
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

    let mut string_doc_result = String::from("");
    for doc_result in read_list_result {
        let (doc, metadata) = match doc_result {
            Err(_e) => {
                println!("cannot read doc");
                continue;
            }
            Ok(r) => r,
        };
        let doc_id = split_name_to_id(&metadata.name);
        let json_doc = serde_json::to_value(&doc).unwrap();
        string_doc_result.push_str(doc_id);
        string_doc_result.push_str(&json_doc.to_string());
    }
    //println!("{string_doc_result}");
    context.insert("read_list_result", &string_doc_result);

    saint_sorting::render(tmpl, &context, "clothing.html")
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
