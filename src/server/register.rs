use actix_web::{web, Scope, Responder};
use super::super::model::user::*;

fn post(body: web::Json<RegisterUser>) -> impl Responder {
    body
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/register/", web::post().to(post))
}