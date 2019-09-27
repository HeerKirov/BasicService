use actix_web::{web, Scope, Responder};
use super::super::model::user::*;
use super::super::db::{get_connection};

fn post(body: web::Json<RegisterUser>) -> impl Responder {
    body
}

fn hello() -> impl Responder {
    let mut s = String::new();
    for row in &(*get_connection()).query("SELECT id, name, username FROM service_user", &[]).unwrap() {
        let id: i32 = row.get("id");
        let username: String = row.get("username");
        let name: String = row.get("name");
        s += &format!("{}, {}, {}", id, username, name);
    }
    s
}

pub fn register_view(scope: Scope) -> Scope {
    scope.route("/register/", web::post().to(post))
        .route("/", web::get().to(hello))
}