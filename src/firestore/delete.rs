use crate::*;
use super::firestore_error::FireStoreError;

pub async fn delete_firestore(
    session: Session,
    document_id: String,
) -> actix_web::Result<(), FireStoreError> {
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i.to_string(),
    };

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let path = user_id + &String::from("/") + &document_id;
    let _result = documents::delete(&auth, &path, true);

    Ok(())

}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbDelete {
    document_id: String,
}
