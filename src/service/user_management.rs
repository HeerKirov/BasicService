use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::user::{CreatePath, ViewManageUser};
use super::user::password_encrypt;

pub fn user_list(t: &Transaction) -> Result<Vec<ViewManageUser>, Error> {
    match t.query("SELECT * FROM service_user WHERE NOT deleted", &[]) {
        Ok(rows) => Ok(rows.iter().map(|row| ViewManageUser {
            enable: row.get("enable"),
            username: row.get("username"),
            name: row.get("name"),
            cover: row.get("cover"),
            is_staff: row.get("is_staff"),
            last_login: row.get("last_login"),
            last_login_ip: row.get("last_login_ip"),
            create_time: row.get("create_time"),
            create_path: {
                let create_path: String = row.get("create_path");
                CreatePath::from(&create_path).unwrap()
            }
        }).collect()),
        Err(e) => Err(e)
    }
}

pub fn user_get(t: &Transaction, user_id: i32) -> Result<Option<ViewManageUser>, Error> {
    match t.query("SELECT * FROM service_user WHERE NOT deleted AND id = $1 LIMIT 1", &[&user_id]) {
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewManageUser {
                enable: rows.get(0).get("enable"),
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
pub fn user_get_by_username(t: &Transaction, username: &String) -> Result<Option<ViewManageUser>, Error> {
    match t.query("SELECT * FROM service_user WHERE NOT deleted AND username = $1 LIMIT 1", &[username]) {
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewManageUser {
                enable: rows.get(0).get("enable"),
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
        }else{ Ok(None) },
        Err(e) => Err(e)
    }
}

pub fn user_set_password(t: &Transaction, username: &String, new_password: &String) -> Result<bool, Error> {
    match t.execute("UPDATE service_user SET password = $2 WHERE username = $1", &[username, &password_encrypt(new_password)]) {
        Ok(size) => Ok(size > 0),
        Err(e) => Err(e)
    }
}

pub fn user_set_enable(t: &Transaction, username: &String, enable: bool) -> Result<bool, Error> {
    match t.execute("UPDATE service_user SET enable = $2 WHERE NOT deleted AND username = $1", &[username, &enable]) {
        Err(e) => Err(e),
        Ok(row) => Ok(row > 0)
    }
}

pub fn user_delete(t: &Transaction, user_id: i32) -> Result<bool, Error> {
    match t.execute("UPDATE service_user SET deleted = true WHERE NOT deleted AND id = $1", &[&user_id]) {
        Err(e) => Err(e),
        Ok(row) => Ok(row > 0)
    }
}