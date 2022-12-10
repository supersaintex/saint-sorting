use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::{self, Key},
    middleware::Logger,
    web, App, HttpRequest, HttpServer, Result, Responder, Error,
    HttpResponse
};

use uuid::Uuid;

use serde_json::json;

// simple index handler with session
pub async fn index(session: Session, req: HttpRequest) 
// -> Result<&'static str> {
-> Result<impl Responder, Error> {
    log::info!("{req:?}");

    // RequestSession trait is used for session access
    // let mut counter = 1;
    // if let Some(count) = session.get::<i32>("counter")? {
    //     log::info!("SESSION value: {count}");
    //     counter = count + 1;
    //     session.insert("counter", counter)?;
    // } else {
    //     session.insert("counter", counter)?;
    // }

    // println!("{}", counter);

    // Ok("welcome!")
    
    let json = match session.get::<Uuid>("user_id")? {
         Some(user_id) => json!({ "user_id": &user_id }),
         None => {
             let user_id = Uuid::new_v4();
             session.insert("user_id", &user_id)?;
 
             json!({"user_id": &user_id })
         }
     };
 
     Ok(HttpResponse::Ok().json(&json))


}