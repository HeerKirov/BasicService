pub mod register;

use actix_web::{web, App, HttpServer, Scope};
use actix_web::middleware::Logger;
use super::util::config::*;

fn register_views(scope: Scope) -> Scope {
    register::register_view(scope)
}

pub fn run_server() {
    let config = get_config();
    let prefix = String::from(config.get(WEB_PREFIX));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("\"%r\" %s - %Ts"))
            .service(register_views(web::scope(&prefix)))
    })
    .bind(format!("0.0.0.0:{}", config.get(WEB_PORT))).expect(&format!("cannot bind to port {}", config.get(WEB_PORT)))
    .run().unwrap();
}