use postgres::transaction::{Transaction};
use postgres::{Error};
use chrono::prelude::{Utc};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use super::super::model::user::{CreatePath};
use super::super::util::config::*;

pub fn user_exists(t: &Transaction, username: &String) -> bool {
    t.query("SELECT username FROM service_user WHERE username = $1", &[username]).unwrap().len() > 0
}

pub fn user_create(t: &Transaction, username: &String, password: &String, name: &String, is_staff: bool, create_path: CreatePath) -> Result<(), Error> {
    match t.execute("INSERT INTO 
        service_user(username, password, name, is_staff, create_time, create_path)
        VALUES($1, $2, $3, $4, $5, $6)", 
        &[username, &password_encrypt(password), name, &is_staff, &Utc::now(), &create_path.to_string()]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
    }
}

pub fn password_encrypt(password: &str) -> String {
    let config = get_config();
    let mut hmac = Hmac::new(Sha1::new(), config.get(SECRET_KEY).as_bytes());
    hmac.input(password.as_bytes());
    let s = hmac.result();
    let mut ret = String::with_capacity(40);
    for b in s.code().iter() {
        ret += &format!("{:02x}", b);
    }
    ret.to_string()
}