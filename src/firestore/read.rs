use crate::*;
use super::firestore_error::FireStoreError;

pub async fn read_firestore<T: for<'a> Deserialize<'a>>(
    session: Session,
    document_id: &str,
) -> Result<T, FireStoreError> {
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i.to_string(),
    };

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let dto : T = documents::read(&auth, &user_id, document_id).unwrap();

    Ok(dto)
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}
