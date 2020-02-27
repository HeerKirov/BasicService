use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::app_use::ViewAppUse;
use super::super::model::app::ViewApp;

pub fn use_list(t: &Transaction, user_id: i32) -> Result<Vec<ViewAppUse>, Error> {
    match t.query("SELECT sau.last_use AS last_use,
                   sau.create_time AS create_time,
                   sa.unique_name AS unique_name,
                   sa.name AS app_name,
                   sa.description AS app_description,
                   sa.url AS app_url,
                   sa.public AS app_public,
                   sa.create_time AS app_create_time,
                   sa.update_time AS app_update_time
            FROM service_app_use sau
            INNER JOIN service_app sa on sau.app_id = sa.id AND NOT deleted AND enable
            WHERE sau.user_id = $1", &[&user_id]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row| ViewAppUse {
            last_use: row.get("last_use"),
            create_time: row.get("create_time"),
            public_app: row.get("app_public"),
            app: ViewApp {
                app_id: row.get("unique_name"),
                name: row.get("app_name"),
                description: row.get("app_description"),
                url: row.get("app_url"),
                create_time: row.get("app_create_time"),
                update_time: row.get("app_update_time")
            }
        }).collect())
    }
}

pub fn use_get(t: &Transaction, user_id: i32, app_id: &String) -> Result<Option<ViewAppUse>, Error> {
    match t.query("SELECT sau.last_use AS last_use,
                   sau.create_time AS create_time,
                   sa.unique_name AS unique_name,
                   sa.name AS app_name,
                   sa.description AS app_description,
                   sa.url AS app_url,
                   sa.public AS app_public,
                   sa.create_time AS app_create_time,
                   sa.update_time AS app_update_time
            FROM service_app_use sau
            INNER JOIN service_app sa on sau.app_id = sa.id AND NOT deleted AND enable
            WHERE sau.user_id = $1 AND sa.unique_name = $2 LIMIT 1", &[&user_id, app_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewAppUse {
                last_use: rows.get(0).get("last_use"),
                create_time: rows.get(0).get("create_time"),
                public_app: rows.get(0).get("app_public"),
                app: ViewApp {
                    app_id: rows.get(0).get("unique_name"),
                    name: rows.get(0).get("app_name"),
                    description: rows.get(0).get("app_description"),
                    url: rows.get(0).get("app_url"),
                    create_time: rows.get(0).get("app_create_time"),
                    update_time: rows.get(0).get("app_update_time")
                }
            }))
        }else{
            Ok(None)
        }
    }
}