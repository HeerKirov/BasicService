use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::service::app_use_management::{use_list_of_app, use_list_of_user, use_get};
use super::super::service::transaction_res;
use super::verify_staff;

fn list_of_user(username: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let username = &username.to_string();
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match use_list_of_user(trans, &username) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}

fn list_of_app(app_id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let app_id = &app_id.to_string();
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match use_list_of_app(trans, &app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}

fn retrieve(path: web::Path<(String, String)>, req: HttpRequest) -> HttpResponse {
    let username = &path.0.to_string();
    let app_id = &path.1.to_string();
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match use_get(trans, &username, &app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok)
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/admin/app/{app_id}/use-user/", web::get().to(list_of_app))
        .route("/admin/user/{username}/use-app/", web::get().to(list_of_user))
        .route("/admin/app-use/{username}/{app_id}/", web::get().to(retrieve))
}