use crate::{
    firestore::{
        firestore_error,
        read::{read_firestore, read_list_firestore},
    },
    *,
};
use firestore_db_and_auth::documents;

use once_cell::sync::Lazy;
use std::sync::Mutex;
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
    println!("read end");

    context.insert("brand", &read_result.brand);
    context.insert("year", &read_result.year);
    context.insert("month", &read_result.month);

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
    fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string();
}

println!("read_list_result's type is:{}", type_of(&read_list_result));

    let mut readList = String::from("");
    for doc_result in read_list_result {
        let (doc, _metadata) = match doc_result {
            Err(_e) => {
                println!("cannot read doc");
                continue;
            }
            Ok(r) => r,
        };
    
        let doc_string = doc.to_string();
        let concatenated = format!("brand :{} ,year :{} ,month :{}\n", doc_string.brand, doc_string.year, doc_string.month);
        readList.push_str(&concatenated);
        println!("{}", readList); 
    }
    println!("{}", readList);
    context.insert("readList", &readList);

    


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
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}

impl DTOClothing {
    fn to_string(&self) -> DTOClothing_string {
        DTOClothing_string{
            brand: self.brand.to_string(),
            year: self.year.to_string(),
            month: self.month.to_string(),
        }
    
    }
}
#[derive(Serialize, Deserialize)]
pub struct DTOClothing_string{
    brand: String,
    year: String,
    month: String,
}