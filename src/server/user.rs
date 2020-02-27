use std::error::Error;
use futures::StreamExt;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use actix_multipart::Multipart;
use super::super::model::user::{UpdateUser, UpdatePassword, ViewCover};
use super::super::service::user::{user_get, user_update, user_set_password, user_set_cover};
use super::super::service::image::Image;
use super::super::service::{transaction_res, transaction_result};
use super::verify_login;
use super::super::util::config::*;

fn get(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_get(trans, user_id) {
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => HttpResponse::InternalServerError().body("User model is not found."),
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

fn set(body: web::Json<UpdateUser>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_update(trans, user_id, &body) {
            Ok(_) => if let Some(user) = user_get(trans, user_id).unwrap() {
                HttpResponse::Ok().json(user)
            }else{
                HttpResponse::InternalServerError().body("User model is not found.")
            },
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

fn set_password(body: web::Json<UpdatePassword>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_set_password(trans, user_id, &body.old_password, &body.new_password) {
            Ok(true) => HttpResponse::Ok().body("success"),
            Ok(false) => HttpResponse::Unauthorized().body("Password Wrong"),
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

async fn upload_cover(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let user_id = match transaction_result(move |trans| verify_login(trans, &req)) {
        Ok(user_id) => user_id,
        Err(e) => return e
    };

    let mut image = match Image::new() {
        Ok(image) => image,
        Err(e) => return HttpResponse::InternalServerError().body(e.description().to_string())
    };
    let mut success = false;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
        };
        let disposition = field.content_disposition().unwrap();
        let name = disposition.get_name().unwrap();
        if name == "file" {
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                image.write(&data).unwrap();
            }
            success = true;
            break;
        }
    }
    if !success {
        return HttpResponse::BadRequest().body("Item 'file' is necessary");
    }

    let target = Image::new_filename(user_id);
    let target_name = format!("{}/{}", get_config().get(STATIC_COVER_DIRECTORY), &target);
    if let Err(e) = image.close().convert() { return HttpResponse::BadRequest().body(e.description().to_string()) }
    if let Err(e) = image.move_to(&target_name) { return HttpResponse::InternalServerError().body(e.description().to_string()) }
    image.clear();
    transaction_res(|trans| {
        if let Some(old_cover) = user_get(trans, user_id).unwrap().unwrap().cover {
            if let Err(e) = Image::delete(&format!("{}/{}", get_config().get(STATIC_COVER_DIRECTORY), old_cover)) {
                return HttpResponse::InternalServerError().body(e.description().to_string())
            }
        }
        if let Err(e) = user_set_cover(trans, user_id, &target) {
            HttpResponse::InternalServerError().body(e.description().to_string())
        }else{
            HttpResponse::Ok().json(ViewCover{cover: target.to_string()})
        }
    })
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/user/", web::get().to(get))
        .route("/user/", web::post().to(set))
        .route("/user/", web::put().to(set))
        .route("/user/password/", web::post().to(set_password))
        .route("/user/cover/", web::post().to(upload_cover))
}