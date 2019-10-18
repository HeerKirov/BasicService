use postgres::transaction::Transaction;
use postgres::Error;
use chrono::prelude::Utc;
use uuid::Uuid;
use super::super::model::app::{ViewManageApp, CreateApp};

pub fn app_list(t: &Transaction) -> Result<Vec<ViewManageApp>, Error> {
    match t.query("SELECT id, name, description, public, enable, create_time, update_time 
            FROM service_app WHERE NOT deleted", &[]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row| ViewManageApp {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            public: row.get("public"),
            enable: row.get("enable"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time")
        }).collect())
    }
}

pub fn app_exists(t: &Transaction, name: &String) -> Result<bool, Error> {
    match t.query("SELECT 1 FROM service_app WHERE name = $1 LIMIT 1", &[name]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.len() > 0)
    }
}

pub fn app_create(t: &Transaction, body: &CreateApp) -> Result<(), Error> {
    let now = Utc::now();
    match t.execute("INSERT INTO service_app(name, description, secret, public, create_time, update_time)
            VALUES($1, $2, $3, $4, $5, $6)", 
            &[&body.name, &body.description, &generate_secret(now.timestamp_millis()), &body.public, &now, &now]) {
        Err(e) => Err(e),
        Ok(_) => Ok(())
    }
}

pub fn app_get(t: &Transaction, app_id: i32) -> Result<Option<ViewManageApp>, Error> {
    match t.query("SELECT id, name, description, public, enable, create_time, update_time 
            FROM service_app WHERE NOT deleted AND id = $1 LIMIT 1", &[&app_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewManageApp {
                id: rows.get(0).get("id"),
                name: rows.get(0).get("name"),
                description: rows.get(0).get("description"),
                public: rows.get(0).get("public"),
                enable: rows.get(0).get("enable"),
                create_time: rows.get(0).get("create_time"),
                update_time: rows.get(0).get("update_time")
            }))
        }else{
            Ok(None)
        }
    }
}

pub fn app_get_by_name(t: &Transaction, name: &String) -> Result<Option<ViewManageApp>, Error> {
    match t.query("SELECT id, name, description, public, enable, create_time, update_time 
            FROM service_app WHERE NOT deleted AND name = $1 LIMIT 1", &[name]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewManageApp {
                id: rows.get(0).get("id"),
                name: rows.get(0).get("name"),
                description: rows.get(0).get("description"),
                public: rows.get(0).get("public"),
                enable: rows.get(0).get("enable"),
                create_time: rows.get(0).get("create_time"),
                update_time: rows.get(0).get("update_time")
            }))
        }else{
            Ok(None)
        }
    }
}

pub fn app_get_secret(t: &Transaction, app_id: i32) -> Result<Option<String>, Error> {
    match t.query("SELECT secret FROM service_app WHERE NOT deleted AND id = $1 LIMIT 1", &[&app_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(rows.get(0).get("secret")))
        }else{
            Ok(None)
        }
    }
}

pub fn app_update_secret(t: &Transaction, app_id: i32) -> Result<Option<String>, Error> {
    let now = Utc::now();
    let secret = generate_secret(now.timestamp_millis());
    match t.execute("UPDATE service_app SET secret = $1 WHERE NOT deleted AND id = $2", &[&secret, &app_id]) {
        Err(e) => Err(e),
        Ok(size) => if size > 0 {
            Ok(Some(secret))
        }else{
            Ok(None)
        }
    }
}

pub fn app_update(t: &Transaction, app_id: i32, body: &CreateApp) -> Result<bool, Error> {
    match t.execute("UPDATE service_app SET name = $1, description = $2, public = $3, update_time = $4 
            WHERE id = $5 AND NOT deleted", 
            &[&body.name, &body.description, &body.public, &Utc::now(), &app_id]) {
        Err(e) => Err(e),
        Ok(size) => Ok(size > 0)
    }
}

pub fn app_delete(t: &Transaction, app_id: i32) -> Result<bool, Error> {
    match t.execute("UPDATE service_app SET deleted = true, update_time = $2 
            WHERE id = $1 AND NOT deleted", &[&app_id, &Utc::now()]) {
        Err(e) => Err(e),
        Ok(size) => Ok(size > 0)
    }
}

fn generate_secret(create_time: i64) -> String {
    format!("{}{:024}", Uuid::new_v4().to_simple().to_string(), 1 << 63 ^ create_time)
}