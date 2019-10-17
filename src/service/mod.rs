pub mod user;
pub mod global_setting;
pub mod token;
pub mod image;
pub mod registration_code;

use postgres::transaction::Transaction;
use actix_web::{HttpResponse};
use super::db::get_connection;

pub fn transaction_result<F, OK, ERR>(execution: F) -> Result<OK, ERR> where F: Fn(&Transaction) -> Result<OK, ERR> {
    let conn = &*get_connection();
    let trans = conn.transaction().unwrap();
    let result = execution(&trans);
    match result {
        Ok(_) => trans.set_commit(),
        Err(_) => trans.set_rollback()
    }
    trans.finish().unwrap();
    result
}

pub fn transaction_res<F>(execution: F) -> HttpResponse where F: Fn(&Transaction) -> HttpResponse {
    let conn = &*get_connection();
    let trans = conn.transaction().unwrap();
    let result = execution(&trans);
    let status = result.status();
    if status.is_server_error() {
        trans.set_rollback();
    }else{
        trans.set_commit();
    }
    trans.finish().unwrap();
    result
}

pub fn transaction<F, T>(execution: F) -> T where F: Fn(&Transaction) -> T {
    let conn = &*get_connection();
    let trans = conn.transaction().unwrap();
    let result = execution(&trans);
    trans.commit().unwrap();
    result
}