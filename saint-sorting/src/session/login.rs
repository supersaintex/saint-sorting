use super::*;

pub async fn login(req: HttpRequest) -> HttpResponse {

    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    print!("hello, login");

    HttpResponse::Found()
        .insert_header(("location", "/"))
        .finish()
 
}