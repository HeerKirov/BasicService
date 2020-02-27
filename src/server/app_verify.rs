use log::error;
use std::thread;
use std::error::Error;
use actix_web::{web, Scope, HttpResponse};
use super::super::model::verify::{AppVerifyRequest, AppVerifyResponse, GetInfoRequest, UpdateInfoRequest};
use super::super::service::app_verify::{VerifyError, verify_app_secret, verify_use, update_last_use, get_use_info, update_use_info};
use super::super::service::user::get_id_by_username;
use super::super::service::token::token_get_with_id;
use super::super::service::{transaction_res, transaction};

fn verify(body: web::Json<AppVerifyRequest>) -> HttpResponse {
    transaction_res(|trans| {
        let app_id = match verify_app_secret(trans, &body.secret) {
            Err(VerifyError::SQL(e)) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Err(VerifyError::AppNotEnabled) => return HttpResponse::Forbidden().body("App not enabled"),
            Err(VerifyError::SecretWrong) => return HttpResponse::Unauthorized().body("Secret wrong"),
            Ok(ok) => ok
        };
        let (user_id, username) = match token_get_with_id(trans, &body.token) {
            Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => return HttpResponse::NotFound().body("Token wrong"),
            Ok(Some((user_id, retrieve))) => (user_id, retrieve.username)
        };

        match verify_use(trans, user_id, app_id) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(()) => {
                update_last_use_thread(user_id, app_id);
                HttpResponse::Ok().json(AppVerifyResponse{username})
            }
        }
    })
}

fn get_info(body: web::Json<GetInfoRequest>) -> HttpResponse {
    transaction_res(|trans| {
        let app_id = match verify_app_secret(trans, &body.secret) {
            Err(VerifyError::SQL(e)) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Err(VerifyError::AppNotEnabled) => return HttpResponse::Forbidden().body("App not enabled"),
            Err(VerifyError::SecretWrong) => return HttpResponse::Unauthorized().body("Secret wrong"),
            Ok(ok) => ok
        };
        match get_use_info(trans, app_id, &body.username) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().body("Use info not found"),
            Ok(Some(info)) => HttpResponse::Ok().json(info)
        }
    })
}

fn update_info(body: web::Json<UpdateInfoRequest>) -> HttpResponse {
    transaction_res(|trans| {
        let app_id = match verify_app_secret(trans, &body.secret) {
            Err(VerifyError::SQL(e)) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Err(VerifyError::AppNotEnabled) => return HttpResponse::Forbidden().body("App not enabled"),
            Err(VerifyError::SecretWrong) => return HttpResponse::Unauthorized().body("Secret wrong"),
            Ok(ok) => ok
        };
        let user_id = match get_id_by_username(trans, &body.username) {
            Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => return HttpResponse::NotFound().body("Use info not found"),
            Ok(Some(user_id)) => user_id
        };
        match update_use_info(trans, app_id, user_id, &body.info) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(false) => HttpResponse::NotFound().body("Use info not found"),
            Ok(true) => match get_use_info(trans, app_id, &body.username) {
                Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
                Ok(None) => HttpResponse::NotFound().body("Use info not found"),
                Ok(Some(info)) => HttpResponse::Ok().json(info)
            }
        }
    })
}

fn update_last_use_thread(user_id: i32, app_id: i32) {
    thread::spawn(move|| {
        transaction(move |trans| {
            if let Err(e) = update_last_use(trans, user_id, app_id) {
                error!("update user last use message failed. {}", e)
            }
        })
    });
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/interface/verify/", web::post().to(verify))
        .route("/interface/info/get/", web::post().to(get_info))
        .route("/interface/info/update/", web::post().to(update_info))
}
