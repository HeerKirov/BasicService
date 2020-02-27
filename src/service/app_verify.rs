use postgres::transaction::Transaction;
use postgres::Error;
use chrono::Duration;
use chrono::prelude::Utc;
use super::super::model::verify::InfoResponse;

pub enum VerifyError {
    SQL(Error),
    AppNotEnabled,
    SecretWrong
}

pub fn verify_app_secret(t: &Transaction, secret: &String) -> Result<i32, VerifyError> {
    match t.query("SELECT id, enable FROM service_app WHERE NOT deleted AND secret = $1 LIMIT 1", &[secret]) {
        Err(e) => Err(VerifyError::SQL(e)),
        Ok(rows) => if rows.len() > 0 {
            let enable: bool = rows.get(0).get("enable");
            if enable {
                Ok(rows.get(0).get("id"))
            }else{
                Err(VerifyError::AppNotEnabled)
            }
        }else{
            Err(VerifyError::SecretWrong)
        }
    }
}

pub fn verify_use(t: &Transaction, user_id: i32, app_id: i32) -> Result<(), Error> {
    let rows = t.query("SELECT 0 FROM service_app_use WHERE user_id = $1 AND app_id = $2", &[&user_id, &app_id])?;
    if rows.len() <= 0 {
        let now = Utc::now();
        match t.execute("INSERT INTO service_app_use(user_id, app_id, info, last_use, create_time, update_time)
                        VALUES($1, $2, $3, $4, $5, $6)", &[&user_id, &app_id, &Option::<String>::None, &now, &now, &now]) {
            Err(e) => Err(e),
            Ok(_) => Ok(())
        }
    }else{
        Ok(())
    }
}

pub fn get_use_info(t: &Transaction, app_id: i32, username: &String) -> Result<Option<InfoResponse>, Error> {
    let rows = t.query("SELECT su.username AS username, su.name AS name, sau.info AS info, su.is_staff AS is_staff
            FROM service_app_use sau
            INNER JOIN service_user su ON su.id = sau.user_id
            WHERE NOT su.deleted AND su.enable AND su.username = $1 AND sau.app_id = $2 LIMIT 1", &[username, &app_id])?;
    if rows.len() > 0 {
        Ok(Some(InfoResponse{
            username: rows.get(0).get("username"),
            name: rows.get(0).get("name"),
            is_staff: rows.get(0).get("is_staff"),
            info: rows.get(0).get("info")
        }))
    }else{
        Ok(None)
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