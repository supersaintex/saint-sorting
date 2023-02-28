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

    //This measure is temporary. Should we avoid using clone?
    let tmpl_copy = tmpl.clone();

    //write documents to database
    match write_firestore(session, add_doc_id, &add_obj, tmpl).await {
        Ok(_response) => (),
        Err(error) => return Err(error),
    }

    //show clothing page after write documents.
    let context = Context::new();
    let view = tmpl_copy
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
