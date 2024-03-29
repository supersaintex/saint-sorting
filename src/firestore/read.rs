use super::firestore_error::FireStoreError;
use crate::*;

/*pub async fn read_firestore<T: for<'a> Deserialize<'a>>(
    session: &Session,
    auth: &ServiceSession,
    document_id: &str,
) -> Result<T, FireStoreError> {
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i.to_string(),
    };

    let dto: T = match documents::read(auth, &user_id, document_id) {
        Err(e) => return Err(e.into()),
        Ok(dto) => dto,
    };

    Ok(dto)
}*/

pub async fn read_list_firestore<'b, T: for<'a> Deserialize<'a>>(
    session: &'b Session,
    auth: &'b ServiceSession,
) -> Result<documents::List<'b, T, firestore_db_and_auth::ServiceSession>, FireStoreError> {
    match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(_) => (),
    };

    let email_address = match session.get::<String>("email_address")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i,
    };

    let dto_list: documents::List<T, _> = documents::list(auth, &email_address);

    Ok(dto_list)
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}
