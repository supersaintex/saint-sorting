use crate::firestore::firestore_error;
use crate::*;

pub async fn clothing_write(
    params: web::Form<FormParamsClothing>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, firestore_error::FireStoreError> {
    let add_doc_id = String::from(&params.document_id);

    let add_brand = String::from(&params.brand);
    let add_year = params.year;
    let add_month = params.month;

    let add_obj = DemoDTOClothing {
        brand: add_brand,
        year: add_year,
        month: add_month,
    };

    //write documents to database
    match write_firestore(session, add_doc_id, &add_obj).await {
        Ok(_) => (),
        Err(error) => return Err(error),
    }

    let context = Context::new();
    let view = tmpl
        .render("clothing.html", &context)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsClothing {
    document_id: String,
    brand: String,
    year: u32,
    month: u32,
}

#[derive(Serialize, Deserialize)]
struct DemoDTOClothing {
    brand: String,
    year: u32,
    month: u32,
}
