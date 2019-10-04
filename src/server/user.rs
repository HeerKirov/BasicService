use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::service::user::user_get;
use super::super::service::transaction;
use super::verify_login;

fn get(req: HttpRequest) -> HttpResponse {
    transaction(|trans| {
        let user_id = match verify_login(&trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_get(&trans, user_id) {
            Ok(user) => if let Some(user) = user {
                HttpResponse::Ok().json(user)
            }else{
                HttpResponse::InternalServerError().body("User model is not found.")
            },
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

fn set() -> HttpResponse {
    //TODO 添加set API
    panic!("not implemented")
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/user/", web::get().to(get))
        .route("/user/", web::put().to(set))
}