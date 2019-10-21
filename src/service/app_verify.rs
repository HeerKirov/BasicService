use postgres::transaction::Transaction;
use postgres::rows::Rows;
use postgres::Error;
use chrono::Duration;
use chrono::prelude::Utc;
use super::super::model::app_use::AppVerifyResponse;

pub enum VerifyError {
    SQL(Error),
    NoThisApp,
    AppNotEnabled,
    SecretWrong,
    NeitherIdNorName
}

pub fn verify_app_secret(t: &Transaction, app_id: &Option<i32>, app_unique_name: &Option<String>, secret: &String) -> Result<i32, VerifyError> {
    let query = if let Some(ref app_id) = app_id {
        t.query("SELECT id, secret, enable FROM service_app WHERE NOT deleted AND id = $1 LIMIT 1", &[app_id])
    }else if let Some(ref unique_name) = app_unique_name {
        t.query("SELECT id, secret, enable FROM service_app WHERE NOT deleted AND unique_name = $1 LIMIT 1", &[unique_name])
    }else{
        return Err(VerifyError::NeitherIdNorName)
    };
    match query {
        Err(e) => Err(VerifyError::SQL(e)),
        Ok(rows) => if rows.len() > 0 {
            let enable: bool = rows.get(0).get("enable");
            if enable {
                let db_secret: String = rows.get(0).get("secret");
                if *secret == db_secret {
                    Ok(rows.get(0).get("id"))
                }else{
                    Err(VerifyError::SecretWrong)
                }
            }else{
                Err(VerifyError::AppNotEnabled)
            }
        }else{
            Err(VerifyError::NoThisApp)
        }
    }
}

pub fn verify_use_by_token(t: &Transaction, token: &String, app_id: i32) -> Result<Option<AppVerifyResponse>, Error> {
    verify_use(t.query("SELECT su.id AS user_id, su.username AS username, su.is_staff AS is_staff,
            sau.app_id AS app_id, sau.info AS info FROM service_user su
            INNER JOIN service_token st ON su.id = st.user_id AND st.key = $1
            LEFT JOIN service_app_use sau ON su.id = sau.user_id AND sau.app_id = $2
            LIMIT 1", &[token, &app_id]), t)
}
pub fn verify_use_by_user_id(t: &Transaction, user_id: i32, app_id: i32) -> Result<Option<AppVerifyResponse>, Error> {
    verify_use(t.query("SELECT su.id AS user_id, su.username AS username, su.is_staff AS is_staff,
            sau.app_id AS app_id, sau.info AS info FROM service_user su
            LEFT JOIN service_app_use sau ON su.id = sau.user_id AND sau.app_id = $2
            WHERE su.id = $1
            LIMIT 1", &[&user_id, &app_id]), t)
}
pub fn verify_use_by_username(t: &Transaction, username: &String, app_id: i32) -> Result<Option<AppVerifyResponse>, Error> {
    verify_use(t.query("SELECT su.id AS user_id, su.username AS username, su.is_staff AS is_staff,
            sau.app_id AS app_id, sau.info AS info FROM service_user su
            LEFT JOIN service_app_use sau ON su.id = sau.user_id AND sau.app_id = $2
            WHERE su.username = $1
            LIMIT 1", &[username, &app_id]), t)
}

fn verify_use(case: Result<Rows, Error>, t: &Transaction) -> Result<Option<AppVerifyResponse>, Error> {
    match case {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() <= 0 {
            Ok(None)
        }else{
            let row = rows.get(0);
            let app_id: Option<i32> = row.get("app_id");
            if let None = app_id {
                let user_id: i32 = row.get("user_id");
                let username: String = row.get("username");
                let is_staff: bool = row.get("is_staff");
                let now = Utc::now();
                match t.execute("INSERT INTO service_app_use(user_id, app_id, info, last_use, create_time, update_time)
                        VALUES($1, $2, $3, $4, $5, $6)", &[&user_id, &app_id, &Option::<String>::None, &now, &now, &now]) {
                    Err(e) => Err(e),
                    Ok(_) => Ok(Some(AppVerifyResponse {
                        user_id,
                        username,
                        is_staff,
                        info: None
                    }))
                }
            }else{
                Ok(Some(AppVerifyResponse {
                    user_id: row.get("user_id"),
                    username: row.get("username"),
                    is_staff: row.get("is_staff"),
                    info: row.get("info")
                }))
            }
        }
    }
}

pub fn update_use_info(t: &Transaction, app_id: i32, user_id: i32, info: &Option<String>) -> Result<bool, Error> {
    match t.execute("UPDATE service_app_use SET info = $1, update_time = $4 WHERE app_id = $2 AND user_id = $3",
            &[info, &app_id, &user_id, &Utc::now()]) {
        Err(e) => Err(e),
        Ok(size) => Ok(size > 0)
    }
}

pub fn update_last_use(t: &Transaction, user_id: i32, app_id: i32) -> Result<(), Error> {
    let now = Utc::now();
    match t.execute("UPDATE service_app_use SET last_use = $1 WHERE user_id = $2 AND app_id = $3 AND (last_use IS NULL OR last_use <= $4)",
            &[&now, &user_id, &app_id, &(now - Duration::minutes(1))]) {
        Err(e) => Err(e),
        Ok(_) => Ok(())
    }
}