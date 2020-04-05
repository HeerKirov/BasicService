use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::service::app::{app_list, app_get};
use super::super::service::transaction_res;
use super::verify_login;

fn list(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        match app_list(trans) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}

fn retrieve(app_id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let app_id = &app_id.to_string();
    transaction_res(|trans| {
        match app_get(trans, &app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok)
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/app/", web::get().to(list))
        .route("/app/{app_id}/", web::get().to(retrieve))
}