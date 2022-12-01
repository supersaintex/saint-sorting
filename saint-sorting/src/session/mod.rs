use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key, middleware, web, App, HttpMessage as _, HttpRequest, HttpResponse, HttpServer,
};

pub mod session;
pub mod login;
pub mod logout;