pub mod user;
pub mod global_setting;
pub mod token;

use postgres::transaction::Transaction;
use super::db::get_connection;

pub fn transaction<F, T>(execution: F) -> T where F: Fn(&Transaction) -> T {
    let conn = &*get_connection();
    let trans = conn.transaction().unwrap();
    let result = execution(&trans);
    trans.commit().unwrap();
    result
}