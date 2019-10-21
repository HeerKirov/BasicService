use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::service::app_use::{use_list, use_get};
use super::super::service::transaction_res;
use super::verify_login;

fn list(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(trans, &req) {
            Err(e) => return e,
            Ok(ok) => ok
        };
        match use_list(trans, user_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}

fn retrieve(use_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(trans, &req) {
            Err(e) => return e,
            Ok(ok) => ok
        };
        match use_get(trans, user_id, *use_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok)
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/app-use/", web::get().to(list))
        .route("/app-use/{use_id}/", web::get().to(retrieve))
}