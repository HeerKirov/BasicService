use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::model::global_setting::{ViewGlobalSetting};
use super::super::service::global_setting::{setting_get, setting_set};
use super::super::service::transaction_res;
use super::verify_staff;

fn get(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match setting_get(trans) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(setting) => HttpResponse::Ok().json(setting)
        }
    })
}

fn post(body: web::Json<ViewGlobalSetting>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match setting_set(trans, &body) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(_) => HttpResponse::Ok().json(&*body)
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/admin/setting/", web::get().to(get))
         .route("/admin/setting/", web::post().to(post))
         .route("/admin/setting/", web::put().to(post))
}
