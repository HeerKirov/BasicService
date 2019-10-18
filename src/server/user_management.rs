use std::error::Error;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use super::super::model::user::{CreatePath, CreateManageUser, UpdateManageUser, UpdateManagePassword};
use super::super::service::user::{user_exists, user_create};
use super::super::service::user_management::{user_list, user_get, user_get_by_username, user_set_password, user_set_enable, user_delete};
use super::super::service::token::token_clean_all_by_id;
use super::super::service::transaction_res;
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
fn retrieve(user_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match user_get(trans, *user_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(Some(ok)) => HttpResponse::Ok().json(ok),
            Ok(None) => HttpResponse::NotFound().finish()
        }
    })
}
fn update(user_id: web::Path<i32>, body: web::Json<UpdateManageUser>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        let mut user = match user_get(trans, *user_id) {
            Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => return HttpResponse::NotFound().finish(),
            Ok(Some(ok)) => ok
        };
        if user.enable ^ body.enable {
            if user.enable {
                if let Err(e) = token_clean_all_by_id(trans, *user_id) {
                    return HttpResponse::InternalServerError().body(e.description().to_string())
                }
            }
            match user_set_enable(trans, *user_id, body.enable) {
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
fn delete(user_id: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        if let Err(e) = token_clean_all_by_id(trans, *user_id) {
            return HttpResponse::InternalServerError().body(e.description().to_string())
        }
        match user_delete(trans, *user_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(false) => HttpResponse::NotFound().finish(),
            Ok(true) => HttpResponse::NoContent().finish()
        }
    })
}
fn update_password(user_id: web::Path<i32>, body: web::Json<UpdateManagePassword>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        if let Err(e) = verify_staff(trans, &req) { return e }
        match user_set_password(trans, *user_id, &body.new_password) {
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
        .route("/admin/user/{user_id}/", web::get().to(retrieve))
        .route("/admin/user/{user_id}/", web::put().to(update))
        .route("/admin/user/{user_id}/", web::delete().to(delete))
        .route("/admin/user/{user_id}/password/", web::put().to(update_password))
}