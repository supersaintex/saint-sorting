use crate::*;

use super::firestore_error::FireStoreError;

pub async fn write_firestore<T>(
    session: Session,
    document_id: String,
    obj: &T,
) -> Result<(), FireStoreError>
where
    T: Serialize,
{
    //session check & unwrap user_id
    let _user_id = match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i.to_string(),
    };

    //unwrap email_address
    let email_address = match session.get::<String>("email_address")? {
        None => {
            return Err(FireStoreError::SessionGet(String::from(
                "the email is not found.",
            )))
        }
        Some(j) => j.to_string(),
    };

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let _result = documents::write(
        &auth,
        &email_address,
        Some(document_id),
        &obj,
        documents::WriteOptions::default(),
    );

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbWrite {
    document_id: String,
    a_string: String,
    an_int: u32,
    another_int: u32,
}
