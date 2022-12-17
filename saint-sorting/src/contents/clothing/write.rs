use crate::*;

pub async fn clothing_write(
    params: web::Form<FormParamsClothing>,
    session: Session,
    tmpl: web::Data<Tera>,)
    -> actix_web::Result<HttpResponse, Error> {

    let add_doc_id  =  String::from(&params.document_id);
    
    //Data Transfer Object
    let add_brand = String::from(&params.brand);
    let add_year = params.year;
    let add_month = params.month;

    let add_obj = DemoDTOClothing {brand: add_brand, year: add_year, month: add_month};

    write_firestore(session, add_doc_id, &add_obj, tmpl).await
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