use super::firestore_error::FireStoreError;
use crate::*;

pub async fn delete_firestore(
    session: &Session,
    document_id: String,
) -> actix_web::Result<(), FireStoreError> {
    match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(_) => (),
    };

    let email_address = match session.get::<String>("email_address")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i,
    };

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let path = email_address + &String::from("/") + &document_id;
    let _result = documents::delete(&auth, &path, true);

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete {
    document_id: String,
}
