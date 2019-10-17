use postgres::transaction::Transaction;
use postgres::Error;
use chrono::Duration;
use chrono::prelude::Utc;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use super::super::model::user::{CreatePath, ViewUser, UpdateUser};
use super::super::util::config::*;

pub enum LoginError {
    PasswordWrong,
    UserNotExist,
    UserNotEnabled
}

pub fn user_exists(t: &Transaction, username: &String) -> Result<bool, Error> {
    match t.query("SELECT username FROM service_user WHERE username = $1", &[username]) {
        Ok(rows) => Ok(rows.len() > 0),
        Err(e) => Err(e)
    }
}

pub fn user_get(t: &Transaction, user_id: i32) -> Result<Option<ViewUser>, Error> {
    match t.query("SELECT id, username, name, cover, is_staff, last_login, last_login_ip, create_time, create_path FROM service_user 
        WHERE NOT deleted AND enable AND id = $1 LIMIT 1", &[&user_id]) {
            Ok(rows) => if rows.len() > 0 {
                Ok(Some(ViewUser {
                    id: rows.get(0).get("id"),
                    username: rows.get(0).get("username"),
                    name: rows.get(0).get("name"),
                    cover: rows.get(0).get("cover"),
                    is_staff: rows.get(0).get("is_staff"),
                    last_login: rows.get(0).get("last_login"),
                    last_login_ip: rows.get(0).get("last_login_ip"),
                    create_time: rows.get(0).get("create_time"),
                    create_path: {
                        let create_path: String = rows.get(0).get("create_path");
                        CreatePath::from(&create_path).unwrap()
                    }
                }))
        }else{
            Ok(None)
        },
        Err(e) => Err(e)
    }
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

pub fn user_update(t: &Transaction, user_id: i32, body: &UpdateUser) -> Result<(), Error> {
    match t.execute("UPDATE service_user SET name = $2 WHERE id = $1", &[&user_id, &body.name]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn user_set_password(t: &Transaction, user_id: i32, old_password: &String, new_password: &String) -> Result<bool, Error> {
    let rows = match t.query("SELECT password FROM service_user WHERE id = $1 AND enable AND NOT deleted LIMIT 1", &[&user_id]) {
        Ok(rows) => rows,
        Err(e) => return Err(e)
    };
    if rows.len() == 0 {
        return Ok(false)
    }
    let db_password: String = rows.get(0).get("password");
    if db_password != password_encrypt(&old_password) {
        return Ok(false)
    }
    match t.execute("UPDATE service_user SET password = $2 WHERE id = $1", &[&user_id, &password_encrypt(new_password)]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

pub fn user_set_cover(t: &Transaction, user_id: i32, cover: &String) -> Result<(), Error> {
    match t.execute("UPDATE service_user SET cover = $2 WHERE id = $1", &[&user_id, cover]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn user_update_last_login(t: &Transaction, user_id: i32, ip: &Option<String>) -> Result<bool, Error> {
    let now = Utc::now();
    match t.execute("UPDATE service_user
            SET last_login = $1, last_login_ip = $2
            WHERE id = $3 AND (last_login IS NULL OR $4 >= last_login)", 
            &[&now, ip, &user_id, &(now - Duration::minutes(1))]) {
        Err(e) => Err(e),
        Ok(size) => Ok(size > 0)
    }
}

pub fn user_authenticate(t: &Transaction, username: &String, password: &String) -> Result<i32, LoginError> {
    let rows = t.query("SELECT id, password, enable FROM service_user WHERE username = $1 AND NOT deleted LIMIT 1", 
        &[username]).unwrap();
    if rows.len() == 0 {
        return Err(LoginError::UserNotExist)
    }
    let enable: bool = rows.get(0).get("enable");
    if !enable {
        return Err(LoginError::UserNotEnabled)
    }
    let db_password: String = rows.get(0).get("password");
    if db_password != password_encrypt(&password) {
        return Err(LoginError::PasswordWrong)
    }
    let id: i32 = rows.get(0).get("id");
    Ok(id)
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

impl LoginError {
    pub fn to_info(&self) -> String {
        match self {
            Self::PasswordWrong => "Password wrong",
            Self::UserNotExist => "User not exist",
            Self::UserNotEnabled => "User not enabled"
        }.to_string()
    }
}