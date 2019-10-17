use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::model::registration_code::{CreateRegistrationCode, UpdateRegistrationCode};
use super::super::service::registration_code::{code_list, code_get, code_create, code_update};
use super::super::service::transaction_res;
use super::verify_staff;

fn list(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(&trans, &req) { return e }
        match code_list(&trans) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(rows) => HttpResponse::Ok().json(rows)
        }
    })
}
fn create(body: web::Json<CreateRegistrationCode>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(&trans, &req) { return e }
        match code_create(&trans, &body) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}
fn retrieve(code: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(&trans, &req) { return e }
        match code_get(&trans, *code) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok),
            Ok(None) => HttpResponse::NotFound().finish()
        }
    })
}
fn update(code: web::Path<i32>, body: web::Json<UpdateRegistrationCode>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(&trans, &req) { return e }
        match code_update(&trans, *code, &body) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok),
            Ok(None) => HttpResponse::NotFound().finish()
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/registration-code/", web::get().to(list))
        .route("/registration-code/", web::post().to(create))
        .route("/registration-code/{code}/", web::get().to(retrieve))
        .route("/registration-code/{code}/", web::put().to(update))
}