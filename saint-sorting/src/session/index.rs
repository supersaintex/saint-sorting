use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key, middleware, web, App, HttpMessage as _, HttpRequest, HttpResponse, HttpServer,
};
pub async fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.id().unwrap_or_else(|_| "Anonymous".to_owned())
    )
}

// async fn login(req: HttpRequest) -> HttpResponse {
//     Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

//     HttpResponse::Found()
//         .insert_header(("location", "/"))
//         .finish()
// }

// async fn logout(id: Identity) -> HttpResponse {
//     id.logout();

//     HttpResponse::Found()
//         .insert_header(("location", "/"))
//         .finish()
// }
