use std::error::Error;
use futures::{Stream, Future};
use futures::future::lazy;
use actix_web::{web, Scope, HttpRequest, HttpResponse, Error as ActixError};
use super::super::model::user::{UpdateUser, UpdatePassword, ViewCover};
use super::super::service::user::{user_get, user_update, user_set_password, user_set_cover};
use super::super::service::image::{Image};
use super::super::service::{transaction_res, transaction_result};
use super::verify_login;
use super::super::util::config::*;

fn get(req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(&trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_get(&trans, user_id) {
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => HttpResponse::InternalServerError().body("User model is not found."),
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

fn set(body: web::Json<UpdateUser>, req: HttpRequest) -> HttpResponse {
    transaction_res(|trans| {
        let user_id = match verify_login(&trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_update(&trans, user_id, &body) {
            Ok(_) => if let Some(user) = user_get(&trans, user_id).unwrap() {
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
        let user_id = match verify_login(&trans, &req) {
            Err(e) => return e,
            Ok(user_id) => user_id
        };

        match user_set_password(&trans, user_id, &body.old_password, &body.new_password) {
            Ok(true) => HttpResponse::Ok().body("success"),
            Ok(false) => HttpResponse::Unauthorized().body("Password Wrong"),
            Err(e) => HttpResponse::InternalServerError().body(e.description().to_string())
        }
    })
}

fn upload_cover(payload: web::Payload, req: HttpRequest) -> Box<dyn Future<Item=HttpResponse, Error=ActixError>> {
    let user_id = match transaction_result(move|trans| verify_login(&trans, &req)) { 
        Ok(user_id) => user_id,
        Err(e) => return Box::new(lazy(||e)),
    };

    let image = match Image::new() { 
        Ok(image) => image, 
        Err(e) => return Box::new(lazy(move|| HttpResponse::InternalServerError().body(e.description().to_string())))
    };
    Box::new(payload.map_err(ActixError::from).fold(image, move|mut image, chunk| {
        image.write(&chunk).unwrap();
        Ok::<_, ActixError>(image)
    }).and_then(move|mut image| {
        let target = Image::new_filename(user_id);
        let target_name = format!("{}/{}", get_config().get(STATIC_COVER_DIRECTORY), &target);
        if let Err(e) = image.close().convert() { return HttpResponse::BadRequest().body(e.description().to_string()) }
        if let Err(e) = image.move_to(&target_name) { return HttpResponse::InternalServerError().body(e.description().to_string()) }
        image.clear();
        transaction_res(|trans| {
            if let Some(old_cover) = user_get(&trans, user_id).unwrap().unwrap().cover {
                if let Err(e) = Image::delete(&format!("{}/{}", get_config().get(STATIC_COVER_DIRECTORY), old_cover)) {
                    return HttpResponse::InternalServerError().body(e.description().to_string())
                }
            }
            if let Err(e) = user_set_cover(&trans, user_id, &target) { 
                HttpResponse::InternalServerError().body(e.description().to_string()) 
            }else{
                HttpResponse::Ok().json(ViewCover{cover: target.to_string()})
            }
        })
    }))
}

pub fn register_view(scope: Scope) -> Scope {
    scope
        .route("/user/", web::get().to(get))
        .route("/user/", web::post().to(set))
        .route("/user/", web::put().to(set))
        .route("/user/password/", web::post().to(set_password))
        .route("/user/cover/", web::post().to_async(upload_cover))
}