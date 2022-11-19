pub mod firestore_db;
pub mod api;
pub mod auth_error;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, error,
                cookie::Key, middleware, HttpMessage as _};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};