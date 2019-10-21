use std::error::Error;
use postgres::transaction::Transaction;
use actix_web::{web, Scope, HttpResponse};
use super::super::model::user::{RegisterUser, CreatePath};
use super::super::model::global_setting::RegisterMode;
use super::super::service::global_setting::setting_get;
use super::super::service::registration_code::{code_get_enable, code_use};
use super::super::service::user::{user_create, user_exists};
use super::super::service::transaction_res;

fn post(body: web::Json<RegisterUser>) -> HttpResponse {
    transaction_res(|trans| {
        let setting = match setting_get(trans) { Ok(ok) => ok, Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()) };
        if let RegisterMode::Close = setting.register_mode { return HttpResponse::Forbidden().body("Register closed") }
        if let Some(ref code) = body.code {
            match code_get_enable(trans, code) {
                Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
                Ok(None) => HttpResponse::BadRequest().body("Disabled registration code"),
                Ok(Some(code_id)) => register_new_user(trans, &body, CreatePath::Code, Some(code_id))
            }
        }else if let RegisterMode::Code = setting.register_mode {
            HttpResponse::BadRequest().body("Need registration code")
        }else{
            register_new_user(trans, &body, CreatePath::Public, None)
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/register/", web::post().to(post))
}

fn register_new_user(trans: &Transaction, body: &RegisterUser, create_path: CreatePath, code_id: Option<i32>) -> HttpResponse {
    if body.username.is_empty() {
        return HttpResponse::BadRequest().body("field `username` cannot be empty")
    }
    if body.password.is_empty() {
        return HttpResponse::BadRequest().body("field `password` cannot be empty")
    }
    if body.name.is_empty() {
        return HttpResponse::BadRequest().body("field `name` cannot be empty")
    }
    match user_exists(trans, &body.username) {
        Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()),
        Ok(exist) => if exist {
            return HttpResponse::BadRequest().body("Username exist")
        }
    }
    if let Err(e) = user_create(trans, &body.username, &body.password, &body.name, false, create_path) {
        return HttpResponse::InternalServerError().body(e.description().to_string())
    }
    if let Some(code_id) = code_id {
        if let Err(e) = code_use(trans, code_id, &body.username) { 
            return HttpResponse::InternalServerError().body(e.description().to_string()) 
        }
    }
    HttpResponse::Created().finish()
}