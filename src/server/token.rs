use std::error::Error;
use actix_web::{web, Scope, HttpResponse};
use super::super::model::token::*;
use super::super::service::user::user_authenticate;
use super::super::service::token::{token_create, token_get, token_update};
use super::super::service::global_setting::setting_get;
use super::super::service::transaction;

fn create(body: web::Json<CreateToken>) -> HttpResponse {
    transaction(|trans| {
        match user_authenticate(&trans, &body.username, &body.password) {
            Ok(user_id) => {
                let setting = setting_get(&trans).unwrap();
                match token_create(&trans, user_id, calculate_effective(body.effective, body.effective_unlimit, setting.effective_max, setting.effective_default)) {
                    Ok(token) => HttpResponse::Created().json(token),
                    Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
                }
            },
            Err(e) => HttpResponse::Unauthorized().body(e.to_info())
        }
    })
}
fn retrieve(token: web::Path<String>) -> HttpResponse {
    transaction(|trans| {
        match token_get(&trans, &token) {
            Ok(token_model) => if let Some(token_model) = token_model {
                HttpResponse::Ok().json(token_model)
            }else{
                HttpResponse::NotFound().finish()
            },
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}
fn update(token: web::Path<String>, body: web::Json<UpdateToken>) -> HttpResponse {
    transaction(|trans| {
        match token_update(&trans, &token, body.effective) {
            Ok(token_model) => if let Some(token_model) = token_model {
                HttpResponse::Ok().json(token_model)
            }else{
                HttpResponse::NotFound().finish()
            },
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/token/", web::post().to(create))
        .route("/token/{token}/", web::get().to(retrieve))
        .route("/token/{token}/", web::put().to(update))
}

fn calculate_effective(effective: Option<i64>, effective_unlimit: Option<bool>, effective_max: Option<i64>, effective_default: i64) -> Option<i64> {
    if effective_unlimit.is_some() && effective_unlimit.unwrap() {
        effective_max
    }else if let Some(e) = effective {
        if let Some(max_e) = effective_max {
            if e > max_e {
                Some(max_e)
            }else{
                Some(e)
            }
        }else{
            Some(e)
        }
    }else{
        Some(effective_default)
    }
}