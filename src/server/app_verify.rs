use log::error;
use std::thread;
use std::error::Error;
use actix_web::{web, Scope, HttpResponse};
use super::super::model::app_use::{AppVerifyRequest, InfoUpdateRequest};
use super::super::service::app_verify::{VerifyError, verify_app_secret, verify_use_by_token, verify_use_by_user_id, verify_use_by_username, update_last_use, update_use_info};
use super::super::service::{transaction_res, transaction};

fn verify(body: web::Json<AppVerifyRequest>) -> HttpResponse {
    transaction_res(|trans| {
        let app_id = match verify_app_secret(trans, &body.app_id, &body.app_unique_name, &body.secret) {
            Err(VerifyError::SQL(e)) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Err(VerifyError::NoThisApp) => return HttpResponse::NotFound().body("No this app"),
            Err(VerifyError::AppNotEnabled) => return HttpResponse::Forbidden().body("App not enabled"),
            Err(VerifyError::SecretWrong) => return HttpResponse::Unauthorized().body("Secret wrong"),
            Err(VerifyError::NeitherIdNorName) => return HttpResponse::BadRequest().body("Neither id nor name"),
            Ok(ok) => ok
        };
        let res = if let Some(ref token) = body.token {
            verify_use_by_token(trans, token, app_id)
        }else if let Some(ref user_id) = body.user_id {
            verify_use_by_user_id(trans, *user_id, app_id)
        }else if let Some(ref username) = body.username {
            verify_use_by_username(trans, username, app_id)
        }else{
            return HttpResponse::BadRequest().body("Neither token nor user_id nor username")
        };
        match res {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(None) => HttpResponse::NotFound().body("No this token"),
            Ok(Some(ok)) => {
                update_last_use_thread(ok.user_id, app_id);
                HttpResponse::Ok().json(ok)
            }
        }
    })
}

fn update_info(body: web::Json<InfoUpdateRequest>) -> HttpResponse {
    transaction_res(|trans| {
        let app_id = match verify_app_secret(trans, &body.app_id, &body.app_unique_name, &body.secret) {
            Err(VerifyError::SQL(e)) => return HttpResponse::InternalServerError().body(e.description().to_string()),
            Err(VerifyError::NoThisApp) => return HttpResponse::NotFound().body("No this app"),
            Err(VerifyError::AppNotEnabled) => return HttpResponse::Forbidden().body("App not enabled"),
            Err(VerifyError::SecretWrong) => return HttpResponse::Unauthorized().body("Secret wrong"),
            Err(VerifyError::NeitherIdNorName) => return HttpResponse::BadRequest().body("Neither id nor name"),
            Ok(ok) => ok
        };
        match update_use_info(trans, app_id, body.user_id, &body.info) {
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string()),
            Ok(false) => HttpResponse::NotFound().body("No this user"),
            Ok(true) => HttpResponse::Ok().finish()
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/interface/verify/", web::post().to(verify))
        .route("/interface/info/", web::post().to(update_info))
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