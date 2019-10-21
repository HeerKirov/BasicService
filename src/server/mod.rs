mod register;
mod token;
mod user;
mod app;
mod app_use;
mod registration_code;
mod global_setting;
mod user_management;
mod app_management;
mod app_use_management;
mod app_verify;

use log::error;
use std::error::Error;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Scope, HttpRequest, HttpResponse};
use actix_files::Files;
use postgres::transaction::Transaction;
use actix_web::middleware::Logger;
use super::util::config::*;
use super::service::token::token_get;
use super::service::user::{user_update_last_login, user_get};

fn register_views(scope: Scope) -> Scope {
    let mut s = scope;
    s = register::register_view(s);
    s = token::register_view(s);
    s = user::register_view(s);
    s = app::register_view(s);
    s = app_use::register_view(s);
    s = registration_code::register_view(s);
    s = global_setting::register_view(s);
    s = user_management::register_view(s);
    s = app_management::register_view(s);
    s = app_use_management::register_view(s);
    s = app_verify::register_view(s);
    s
}

pub fn run_server() {
    let config = get_config();
    let prefix = String::from(config.get(WEB_API_PREFIX));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("[%a] \"%r\" %s - %Ts"))
            .wrap(Cors::new().send_wildcard().max_age(3600))
            .service(Files::new(config.get(STATIC_COVER_PREFIX), config.get(STATIC_COVER_DIRECTORY)).show_files_listing())
            .service(register_views(web::scope(&prefix)))
    })
    .bind(format!("0.0.0.0:{}", config.get(WEB_PORT))).expect(&format!("cannot bind to port {}", config.get(WEB_PORT)))
    .run().unwrap();
}

pub fn verify_login(trans: &Transaction, req: &HttpRequest) -> Result<i32, HttpResponse> {
    verify_permission(trans, req, false)
}
pub fn verify_staff(trans: &Transaction, req: &HttpRequest) -> Result<i32, HttpResponse> {
    verify_permission(trans, req, true)
}
fn verify_permission(trans: &Transaction, req: &HttpRequest, is_staff: bool) -> Result<i32, HttpResponse> {
    //拿到header条目
    let value = if let Some(value) = req.headers().get("Authorization") { value }else{ return Err(HttpResponse::Unauthorized().body("No authorization token.")) };
    //尝试将header解析成str
    let s = match value.to_str() { Ok(s) => s, Err(_) => return Err(HttpResponse::Unauthorized().body("Header authorization cannot be cast to string.")) };

    //判断header是否是正确的Bearer Token的格式
    if !(s.starts_with("Bearer") && s.len() >= 7) { return Err(HttpResponse::Unauthorized().body("Header authorization must be Bearer token.")) }
    //获得token内容
    let token = s[7..].to_string();
    //尝试通过服务层获得token的model
    let token_model = match token_get(trans, &token) { Ok(token_model) => token_model, Err(e) => return Err(HttpResponse::InternalServerError().body(e.description().to_string())) };
    //如果model是None表示不存在此token
    let model = if let Some(model) = token_model { model }else{ return Err(HttpResponse::Unauthorized().body("Authentication token is not exist.")) };
    //在需求是staff的情况下，继续验明user身份
    if is_staff {
        let user_model = match user_get(trans, model.user_id) { Ok(user_model) => user_model, Err(e) => return Err(HttpResponse::InternalServerError().body(e.description().to_string())) };
        let user = if let Some(model) = user_model { model }else{ return Err(HttpResponse::Unauthorized().body("User is not exist.")) };
        if !user.is_staff {
            return Err(HttpResponse::Forbidden().body("Permission denied."));
        }
    }
    //尝试更新最后登录信息
    if let Err(e) = user_update_last_login(trans, model.user_id, &get_request_ip(req)) { error!("update user last login message failed. {}", e) }
    Ok(model.user_id)
}

pub fn get_request_ip(req: &HttpRequest) -> Option<String> {
    if let Some(remote) = req.connection_info().remote() { Some(remote.to_string()) }else{ None }
}