use super::*;

async fn logout(id: Identity) -> HttpResponse {
    id.logout();

    HttpResponse::Found()
        .insert_header(("location", "/"))
        .finish()
}