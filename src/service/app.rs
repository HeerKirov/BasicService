use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::app::ViewApp;

pub fn app_list(t: &Transaction) -> Result<Vec<ViewApp>, Error> {
    match t.query("SELECT unique_name, name, description, url, create_time, update_time
            FROM service_app WHERE NOT deleted AND enable AND public", &[]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row| ViewApp {
            app_id: row.get("unique_name"),
            name: row.get("name"),
            description: row.get("description"),
            url: row.get("url"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time")
        }).collect())
    }
}

pub fn app_get(t: &Transaction, app_id: &String) -> Result<Option<ViewApp>, Error> {
    match t.query("SELECT unique_name, name, description, url, create_time, update_time
            FROM service_app WHERE NOT deleted AND enable AND public AND unique_name = $1 LIMIT 1", &[app_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewApp {
                app_id: rows.get(0).get("unique_name"),
                name: rows.get(0).get("name"),
                description: rows.get(0).get("description"),
                url: rows.get(0).get("url"),
                create_time: rows.get(0).get("create_time"),
                update_time: rows.get(0).get("update_time")
            }))
        }else{
            Ok(None)
        }
    }
}
