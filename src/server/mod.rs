pub mod register;
pub mod token;
pub mod user;
pub mod registration_code;

use std::error::Error;
use actix_web::{web, App, HttpServer, Scope, HttpRequest, HttpResponse};
use postgres::transaction::Transaction;
use actix_web::middleware::Logger;
use super::util::config::*;
use super::service::token::token_get;

fn register_views(scope: Scope) -> Scope {
    let mut s = scope;
    s = register::register_view(s);
    s = token::register_view(s);
    s = user::register_view(s);
    s
}

pub fn run_server() {
    let config = get_config();
    let prefix = String::from(config.get(WEB_PREFIX));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("[%a] \"%r\" %s - %Ts"))
            .service(register_views(web::scope(&prefix)))
    })
    .bind(format!("0.0.0.0:{}", config.get(WEB_PORT))).expect(&format!("cannot bind to port {}", config.get(WEB_PORT)))
    .run().unwrap();
}

pub fn verify_login(trans: &Transaction, req: &HttpRequest) -> Result<i32, HttpResponse> {
    //TODO 添加对user的last_login的更改
    if let Some(value) = req.headers().get("Authorization") {
        match value.to_str() {
            Ok(s) => if s.starts_with("Bearer") && s.len() >= 7 {
                let token = s[7..].to_string();
                match token_get(&trans, &token) {
                    Ok(token_model) => if let Some(token_model) = token_model {
                        Ok(token_model.user_id)
                    }else{
                        Err(HttpResponse::Unauthorized().body("Authentication token is not exist."))
                    },
                    Err(e) => Err(HttpResponse::InternalServerError().body(e.description().to_string()))
                }
            }else{
                Err(HttpResponse::Unauthorized().body("Header authorization must be Bearer token."))
            },
            Err(_) => Err(HttpResponse::Unauthorized().body("Header authorization cannot be cast to string."))
        }
    }else{
        Err(HttpResponse::Unauthorized().body("No authorization token."))
    }
}