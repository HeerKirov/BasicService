use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::app::ViewApp;

pub fn app_list(t: &Transaction) -> Result<Vec<ViewApp>, Error> {
    match t.query("SELECT id, name, description, create_time, update_time
            FROM service_app WHERE NOT deleted AND enable AND public", &[]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row| ViewApp {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time")
        }).collect())
    }
}

pub fn app_get(t: &Transaction, app_id: i32) -> Result<Option<ViewApp>, Error> {
    match t.query("SELECT id, name, description, create_time, update_time
            FROM service_app WHERE NOT deleted AND enable AND public AND id = $1 LIMIT 1", &[&app_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewApp {
                id: rows.get(0).get("id"),
                name: rows.get(0).get("name"),
                description: rows.get(0).get("description"),
                create_time: rows.get(0).get("create_time"),
                update_time: rows.get(0).get("update_time")
            }))
        }else{
            Ok(None)
        }
    }
}
