use postgres::transaction::Transaction;
use postgres::Error;
use chrono::prelude::{Utc, DateTime};
use chrono::Duration;
use uuid::Uuid;
use super::super::model::token::RetrieveToken;

pub fn token_create(t: &Transaction, user_id: i32, username: &String, effective: Option<i64>) -> Result<RetrieveToken, Error> {
    let now = Utc::now();
    let token = RetrieveToken {
        key: uuid_create(user_id, now.timestamp_millis()),
        username: username.to_string(),
        expire_time: if let Some(effective) = effective {
            Some(now + Duration::milliseconds(effective))
        }else{
            None
        },
        create_time: now.clone(),
        update_time: now.clone()
    };
    match t.execute("INSERT INTO service_token(key, user_id, expire_time, create_time, update_time)
        VALUES($1, $2, $3, $4, $5)", 
        &[&token.key, &user_id, &token.expire_time, &token.create_time, &token.update_time]) {
            Ok(_) => Ok(token),
            Err(e) => Err(e)
    }
}

pub fn token_get(t: &Transaction, token_key: &String) -> Result<Option<RetrieveToken>, Error> {
    match token_get_with_id(t, token_key)? {
        Some((_, retrieve)) => Ok(Some(retrieve)),
        None => Ok(None)
    }
}

pub fn token_get_with_id(t: &Transaction, token_key: &String) -> Result<Option<(i32, RetrieveToken)>, Error> {
    match t.query("SELECT su.id AS user_id, su.username AS username, st.expire_time AS expire_time, st.create_time AS create_time, st.update_time AS update_time
        FROM service_token st INNER JOIN service_user su ON st.user_id = su.id WHERE NOT su.deleted AND su.enable AND st.key = $1 LIMIT 1", &[token_key]) {
            Ok(rows) => {
                if rows.len() > 0 {
                    let key = token_key.to_string();
                    let expire_time: Option<DateTime<Utc>> = rows.get(0).get("expire_time");
                    let now = Utc::now();
                    if expire_time.is_some() && expire_time.unwrap() < now {
                        match t.execute("DELETE FROM service_token WHERE key = $1", &[&key]) {
                            Ok(_) => Ok(None),
                            Err(e) => Err(e)
                        }
                    }else{
                        Ok(Some((rows.get(0).get("user_id"), RetrieveToken {
                            key,
                            username: rows.get(0).get("username"),
                            expire_time,
                            create_time: rows.get(0).get("create_time"),
                            update_time: rows.get(0).get("update_time")
                        })))
                    }
                }else{
                    Ok(None)
                }
            },
            Err(e) => Err(e)
    }
}

pub fn token_update(t: &Transaction, token_key: &String, effective: i64) -> Result<Option<RetrieveToken>, Error> {
    match token_get(t, token_key) {
        Ok(token_model) => if let Some(token_model) = token_model {
            let now = Utc::now();
            let expire_time = now + Duration::milliseconds(effective);
            match t.execute("UPDATE service_token SET expire_time = $1, update_time = $2 WHERE key = $3", &[&expire_time, &now, token_key]) {
                Ok(_) => Ok(Some(RetrieveToken {
                    key: token_model.key,
                    username: token_model.username,
                    expire_time: Some(expire_time),
                    create_time: token_model.create_time,
                    update_time: now
                })),
                Err(e) => Err(e)
            }
        }else{
            Ok(None)
        },
        e@Err(_) => e
    }
    
}

pub fn token_clean_expired(t: &Transaction) -> Result<u64, Error> {
    t.execute("DELETE FROM service_token WHERE expire_time IS NOT NULL AND expire_time < $1", &[&Utc::now()])
}

pub fn token_clean_all(t: &Transaction, username: &String) -> Result<u64, Error> {
    match t.query("SELECT id FROM service_user WHERE username = $1 LIMIT 1", &[username]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            token_clean_all_by_id(t, rows.get(0).get("id"))
        }else{
            Ok(0)
        }
    }
}

pub fn token_clean_all_by_id(t: &Transaction, user_id: i32) -> Result<u64, Error> {
    t.execute("DELETE FROM service_token WHERE user_id = $1", &[&user_id])
}

fn uuid_create(user_id: i32, create_time: i64) -> String {
    format!("{:020}{}{:012}", create_time, Uuid::new_v4().to_simple().to_string(), user_id)
}