use postgres::transaction::Transaction;
use postgres::Error;
use chrono::prelude::{Utc, DateTime};
use uuid::Uuid;
use super::super::model::registration_code::{ViewRegistrationCode, CreateRegistrationCode, UpdateRegistrationCode};

pub fn code_list(t: &Transaction) -> Result<Vec<ViewRegistrationCode>, Error> {
    match t.query("SELECT * FROM service_registration_code ORDER BY id DESC", &[]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row|ViewRegistrationCode{
                id: row.get("id"),
                code: row.get("code"),
                enable: row.get("enable"),
                deadline: row.get("deadline"),
                used_time: row.get("used_time"),
                used_user: row.get("used_user"),
                create_time: row.get("create_time")
        }).collect())
    }
}

pub fn code_create(t: &Transaction, body: &CreateRegistrationCode) -> Result<ViewRegistrationCode, Error> {
    let code = Uuid::new_v4().to_simple().to_string();
    match t.execute("INSERT INTO service_registration_code(code, enable, deadline, used_time, used_user, create_time)
            VALUES($1, $2, $3, $4, $5, $6)", 
            &[&code, &true, &body.deadline, &Option::<DateTime<Utc>>::None, &Option::<String>::None, &Utc::now()]) {
        Err(e) => Err(e),
        Ok(_) => Ok(code_get_by_code(t, &code).unwrap().unwrap())
    }
}

pub fn code_get(t: &Transaction, code_id: i32) -> Result<Option<ViewRegistrationCode>, Error> {
    match t.query("SELECT * FROM service_registration_code WHERE id = $1 LIMIT 1", &[&code_id]) {
        Err(e) => Err(e),
        Ok(row) => if row.len() > 0 {
            Ok(Some(ViewRegistrationCode{
                id: row.get(0).get("id"),
                code: row.get(0).get("code"),
                enable: row.get(0).get("enable"),
                deadline: row.get(0).get("deadline"),
                used_time: row.get(0).get("used_time"),
                used_user: row.get(0).get("used_user"),
                create_time: row.get(0).get("create_time")
            }))
        }else{ Ok(None) }
    }
}

fn code_get_by_code(t: &Transaction, code: &String) -> Result<Option<ViewRegistrationCode>, Error> {
    match t.query("SELECT * FROM service_registration_code WHERE code = $1 LIMIT 1", &[code]) {
        Err(e) => Err(e),
        Ok(row) => if row.len() > 0 {
                Ok(Some(ViewRegistrationCode{
                    id: row.get(0).get("id"),
                    code: row.get(0).get("code"),
                    enable: row.get(0).get("enable"),
                    deadline: row.get(0).get("deadline"),
                    used_time: row.get(0).get("used_time"),
                    used_user: row.get(0).get("used_user"),
                    create_time: row.get(0).get("create_time")
                }))
        }else{ Ok(None) }
    }
}

pub fn code_update(t: &Transaction, code_id: i32, body: &UpdateRegistrationCode) -> Result<Option<ViewRegistrationCode>, Error> {
    let model = match code_get(t, code_id) { Ok(Some(ok)) => ok, Ok(None) => return Ok(None), e@Err(_) => return e };
    if !model.enable { return Ok(Some(model)) }

    if body.enable.is_some() && !body.enable.unwrap() {
        match t.execute("UPDATE service_registration_code SET enable = false WHERE id = $1", &[&code_id]) {
            Err(e) => Err(e),
            Ok(_) => Ok(Some(ViewRegistrationCode{
                id: model.id,
                code: model.code,
                enable: false,
                deadline: model.deadline,
                used_time: model.used_time,
                used_user: model.used_user,
                create_time: model.create_time
            }))
        }
    }else{
        match t.execute("UPDATE service_registration_code SET deadline = $2 WHERE id = $1", &[&code_id, &body.deadline]) {
            Err(e) => Err(e),
            Ok(_) => Ok(Some(ViewRegistrationCode{
                id: model.id,
                code: model.code,
                enable: model.enable,
                deadline: body.deadline,
                used_time: model.used_time,
                used_user: model.used_user,
                create_time: model.create_time
            }))
        }
    }
}

pub fn code_get_enable(t: &Transaction, code: &String) -> Result<Option<i32>, Error> {
    match t.query("SELECT id, enable, deadline FROM service_registration_code WHERE code = $1 LIMIT 1", &[code]) {
        Err(e) => Err(e),
        Ok(row) => if row.len() > 0 {
            let id: i32 = row.get(0).get("id");
            let enable: bool = row.get(0).get("enable");
            let deadline: Option<DateTime<Utc>> = row.get(0).get("deadline");
            if enable {
                if let Some(deadline) = deadline {
                    if deadline < Utc::now() {
                        if let Err(e) = t.execute("UPDATE service_registration_code SET enable = false WHERE id = $1", &[&id]) {
                            Err(e)
                        }else{
                            Ok(None)
                        }
                    }else{ Ok(Some(id)) }
                }else{ Ok(Some(id)) }
            }else{ Ok(None) }
        }else{ Ok(None) }
    }
}

pub fn code_use(t: &Transaction, code_id: i32, username: &String) -> Result<(), Error> {
    match t.execute("UPDATE service_registration_code 
            SET enable = false, used_time = $2, used_user = $3 WHERE id = $1", 
            &[&code_id, &Utc::now(), username]) {
        Err(e) => Err(e),
        Ok(_) => Ok(())
    }
}