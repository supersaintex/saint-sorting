use crate::firestore::{firestore_error, write::write_firestore};
use crate::*;

use super::read::clothing_read_list_render;

pub async fn clothing_write(
    params: web::Form<FormParamsClothing>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> actix_web::Result<HttpResponse, Error> {
    let add_doc_id = String::from(&params.document_id);

    let add_brand = String::from(&params.brand);
    let add_year = params.year;
    let add_month = params.month;
    let add_season = String::from(&params.season);
    let add_shop = String::from(&params.shop);
    let add_category = String::from(&params.category);

    let add_obj = DemoDTOClothing {
        brand: add_brand,
        year: add_year,
        month: add_month,
        season: add_season,
        shop: add_shop,
        category: add_category,
    };

    let mut context = Context::new();
    //write documents to database
    match write_firestore(&session, add_doc_id, &add_obj).await {
        Ok(_) => (),
        Err(firestore_error::FireStoreError::Firebase(e)) => {
            context.insert("failure_message_write", "writing failed...");
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
    }
    clothing_read_list_render(&session, tmpl, &mut context).await
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsClothing {
    document_id: String,
    brand: String,
    year: u32,
    month: u32,
    season: String,
    shop: String,
    category: String,
}

#[derive(Serialize, Deserialize)]
struct DemoDTOClothing {
    brand: String,
    year: u32,
    month: u32,
    season: String,
    shop: String,
    category: String,
}
