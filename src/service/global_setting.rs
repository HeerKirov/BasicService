use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::global_setting::{RegisterMode, ViewGlobalSetting};

pub fn setting_create(t: &Transaction, register_mode: RegisterMode, effective_max: Option<i64>, effective_default: i64) -> Result<(), Error> {
    if t.query("SELECT 1 FROM service_global_setting", &[]).unwrap().len() == 0 {
        match t.execute("INSERT INTO 
            service_global_setting(register_mode, effective_max, effective_default)
            VALUES($1, $2, $3)", &[&register_mode.to_string(), &effective_max, &effective_default]) {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
        }
    }else{
        setting_set(t, &ViewGlobalSetting{
            register_mode: register_mode, 
            effective_max: effective_max, 
            effective_default: effective_default
        })
    }   
}

pub fn setting_set(t: &Transaction, body: &ViewGlobalSetting) -> Result<(), Error> {
    match t.execute("UPDATE service_global_setting
        SET register_mode = $1, effective_max = $2, effective_default = $3 WHERE TRUE", 
        &[&body.register_mode.to_string(), &body.effective_max, &body.effective_default]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
    }
}

pub fn setting_get(t: &Transaction) -> Result<ViewGlobalSetting, Error> {
    match t.query("SELECT * FROM service_global_setting LIMIT 1", &[]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(ViewGlobalSetting {
            register_mode: RegisterMode::from_string(&rows.get(0).get("register_mode")).unwrap(),
            effective_default: rows.get(0).get("effective_default"),
            effective_max: rows.get(0).get("effective_max")
        })
    }
}