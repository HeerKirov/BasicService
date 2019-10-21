use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::service::app_use_management::{use_list_of_app, use_list_of_user, use_get_by_id};
use super::super::service::transaction_res;
use super::verify_staff;

fn list_of_user(user_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match use_list_of_user(trans, *user_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}

fn list_of_app(app_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match use_list_of_app(trans, *app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}

fn retrieve(use_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match use_get_by_id(trans, *use_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok)
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/admin/app/{app_id}/use/", web::get().to(list_of_app))
        .route("/admin/user/{user_id}/use/", web::get().to(list_of_user))
        .route("/admin/app-use/{use_id}/", web::get().to(retrieve))
}