use crate::*;
use super::firestore_error::FireStoreError;

pub async fn read_firestore(
    session: Session,
    document_id: String,
) -> actix_web::Result<(), FireStoreError> {
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Err(FireStoreError::SessionGet(String::from("unauthorized"))),
        Some(i) => i.to_string(),
    };

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let obj: MyDTO = documents::read(&auth, &user_id, document_id).unwrap();

    println!("read start");
    println!("{}", obj.a_string);
    println!("{}", obj.an_int);
    println!("{}", obj.another_int);
    println!("read end");

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbRead {
    document_id: String,
}

#[derive(Serialize, Deserialize)]
struct MyDTO {
    a_string: String,
    an_int: i32,
    another_int: i32,
}
