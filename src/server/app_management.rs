use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::model::app::{CreateApp, UpdateApp, ViewManageSecret};
use super::super::service::app_management::{app_list, app_create, app_get, app_get_by_unique_name, app_get_secret, app_update_secret, app_exists, app_update, app_delete};
use super::super::service::app_use_management::use_delete_by_app;
use super::super::service::transaction_res;
use super::super::util::check::validate_std_name;
use super::verify_staff;

fn list(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match app_list(trans) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}
fn create(body: web::Json<CreateApp>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        if !validate_std_name(&body.unique_name) {
            return HttpResponse::BadRequest().body("App unique name is invalid")
        }
        match app_exists(trans, &body.unique_name) {
            Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(true) => return HttpResponse::BadRequest().body("App unique name exist"),
            _ => {}
        }
        match app_create(trans, &body) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(_) => match app_get_by_unique_name(trans, &body.unique_name) {
                Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
                Ok(None) => HttpResponse::InternalServerError().body("App not found."),
                Ok(Some(ok)) => HttpResponse::Created().json(ok)
            }
        }
    })
}
fn retrieve(app_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match app_get(trans, *app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok)
        }
    })
}
fn update(app_id: web::Path<i32>, body: web::Json<UpdateApp>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match app_update(trans, *app_id, &body) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(false) => HttpResponse::NotFound().finish(),
            Ok(true) => match app_get(trans, *app_id) {
                Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
                Ok(None) => HttpResponse::InternalServerError().body("App not found"),
                Ok(Some(ok)) => HttpResponse::Ok().json(ok)
            }
        }
    })
}
fn delete(app_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        if let Err(e) = use_delete_by_app(trans, *app_id) {
            return HttpResponse::InternalServerError().body(e.description().to_string())
        }
        match app_delete(trans, *app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(false) => HttpResponse::NotFound().finish(),
            Ok(true) => HttpResponse::NoContent().finish()
        }
    })
}
fn retrieve_secret(app_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match app_get_secret(trans, *app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ViewManageSecret{ secret: ok})
        }
    })
}
fn update_secret(app_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match app_update_secret(trans, *app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => HttpResponse::Ok().json(ViewManageSecret{ secret: ok})
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/admin/app/", web::get().to(list))
        .route("/admin/app/", web::post().to(create))
        .route("/admin/app/{app_id}/", web::get().to(retrieve))
        .route("/admin/app/{app_id}/", web::put().to(update))
        .route("/admin/app/{app_id}/", web::delete().to(delete))
        .route("/admin/app/{app_id}/secret/", web::get().to(retrieve_secret))
        .route("/admin/app/{app_id}/secret/", web::put().to(update_secret))
}