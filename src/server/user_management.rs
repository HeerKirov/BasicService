use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::model::user::{CreatePath, CreateManageUser, UpdateManageUser, UpdateManagePassword};
use super::super::service::user::{user_exists, user_create};
use super::super::service::user_management::{user_list, user_get_by_username, user_set_password, user_set_enable};
use super::super::service::token::token_clean_all;
use super::super::service::transaction_res;
use super::super::util::check::validate_std_name;
use super::verify_staff;

fn list(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match user_list(trans) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(ok) => HttpResponse::Ok().json(ok)
        }
    })
}
fn create(body: web::Json<CreateManageUser>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        if body.username.is_empty() {
            return HttpResponse::BadRequest().body("field `username` cannot be empty")
        }
        if !validate_std_name(&body.username) {
            return HttpResponse::BadRequest().body("field `username` is invalid")
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
        if let Err(e) = user_create(trans, &body.username, &body.password, &body.name, body.is_staff, CreatePath::Admin) {
            return HttpResponse::InternalServerError().body(e.description().to_string())
        }
        match user_get_by_username(trans, &body.username) {
            Ok(Some(ok)) => HttpResponse::Created().json(ok),
            Ok(None) => HttpResponse::InternalServerError().body("user not found."),
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}
fn retrieve(username: web::Path<String>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match user_get_by_username(trans, &*username) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok),
            Ok(None) => HttpResponse::NotFound().finish()
        }
    })
}
fn update(username: web::Path<String>, body: web::Json<UpdateManageUser>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let username = &username.to_string();
        if let Err(e) = verify_staff(trans, &req) { return e }
        let mut user = match user_get_by_username(trans, &username) {
            Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => return HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => ok
        };
        if user.enable ^ body.enable {
            if user.enable {
                if let Err(e) = token_clean_all(trans, &username) {
                    return HttpResponse::InternalServerError().body(e.description().to_string())
                }
            }
            match user_set_enable(trans, &username, body.enable) {
                Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
                Ok(_) => {
                    user.enable = body.enable;
                    HttpResponse::Ok().json(user)
                }
            }
        }else{
            HttpResponse::Ok().json(user)
        }
    })
}
fn update_password(username: web::Path<String>, body: web::Json<UpdateManagePassword>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match user_set_password(trans, &*username, &body.new_password) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(false) => HttpResponse::NotFound().finish(),
            Ok(true) => HttpResponse::Ok().finish()
        }
    })
}


pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/admin/user/", web::get().to(list))
        .route("/admin/user/", web::post().to(create))
        .route("/admin/user/{username}/", web::get().to(retrieve))
        .route("/admin/user/{username}/", web::put().to(update))
        .route("/admin/user/{username}/password/", web::put().to(update_password))
}