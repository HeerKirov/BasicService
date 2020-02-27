use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::app_use::{ViewUseOfApp, ViewUseOfUser, ViewUse};
use super::super::model::app::ViewManageApp;
use super::super::model::user::{ViewManageUser, CreatePath};

pub fn use_list_of_user(t: &Transaction, username: &String) -> Result<Vec<ViewUseOfUser>, Error> {
    match t.query("SELECT sau.last_use AS last_use, sau.create_time AS create_time, sau.update_time AS update_time,
            sa.unique_name AS unique_name, sa.name AS app_name, sa.description AS app_description, sa.url AS app_url,
            sa.public AS app_public, sa.enable AS app_enable, sa.create_time AS app_create_time, sa.update_time AS app_update_time
            FROM service_app_use sau
            INNER JOIN service_app sa on sau.app_id = sa.id
            INNER JOIN service_user su on sau.user_id = su.id
            WHERE NOT su.deleted AND su.username = $1", &[username]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row|ViewUseOfUser {
            last_use: row.get("last_use"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time"),
            app: ViewManageApp {
                app_id: row.get("unique_name"),
                name: row.get("app_name"),
                description: row.get("app_description"),
                url: row.get("app_url"),
                public: row.get("app_public"),
                enable: row.get("app_enable"),
                create_time: row.get("app_create_time"),
                update_time: row.get("app_update_time")
            }
        }).collect())
    }
}

pub fn use_list_of_app(t: &Transaction, app_id: &String) -> Result<Vec<ViewUseOfApp>, Error> {
    match t.query("SELECT sau.last_use AS last_use, sau.create_time AS create_time, sau.update_time AS update_time,
            su.username AS u_username, su.name AS u_name, su.cover AS u_cover, su.is_staff AS u_is_staff,
            su.last_login AS u_last_login, su.last_login_ip AS u_last_login_ip,
            su.create_time AS u_create_time, su.create_path AS u_create_path, su.enable AS u_enable
            FROM service_app_use sau
            INNER JOIN service_user su ON su.id = sau.user_id
            INNER JOIN service_app sa ON sa.id = sau.app_id
            WHERE NOT su.deleted AND sa.unique_name = $1", &[app_id]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row|ViewUseOfApp {
            last_use: row.get("last_use"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time"),
            user: ViewManageUser {
                username: row.get("u_username"),
                name: row.get("u_name"),
                cover: row.get("u_cover"),
                is_staff: row.get("u_is_staff"),
                last_login: row.get("u_last_login"),
                last_login_ip: row.get("u_last_login_ip"),
                create_time: row.get("u_create_time"),
                create_path:  {
                    let create_path: String = row.get("u_create_path");
                    CreatePath::from(&create_path).unwrap()
                },
                enable: row.get("u_enable")
            }
        }).collect())
    }
}

pub fn use_get(t: &Transaction, username: &String, app_id: &String) -> Result<Option<ViewUse>, Error> {
    match t.query("SELECT sau.last_use AS last_use, sau.create_time AS create_time, sau.update_time AS update_time, sa.unique_name AS unique_name, su.username AS username
            FROM service_app_use sau
            INNER JOIN service_app sa ON sa.id = sau.app_id
            INNER JOIN service_user su ON su.id = sau.user_id
            WHERE NOT su.deleted AND su.username = $1 AND sa.unique_name = $2 LIMIT 1", &[username, app_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewUse {
                last_use: rows.get(0).get("last_use"),
                create_time: rows.get(0).get("create_time"),
                update_time: rows.get(0).get("update_time"),
                username: rows.get(0).get("username"),
                app_id: rows.get(0).get("unique_name")
            }))
        }else{
            Ok(None)
        }
    }
}

pub fn use_delete_by_app(t: &Transaction, app_id: &String) -> Result<(), Error> {
    let id: i32 = match t.query("SELECT id FROM service_app WHERE unique_name = $1 LIMIT 1", &[app_id]) {
        Ok(rows) => if rows.len() > 0 {
            rows.get(0).get("id")
        }else{
            return Ok(())
        },
        Err(e) => return Err(e)
    };
    match t.execute("DELETE FROM service_app_use WHERE app_id = $1", &[&id]) {
        Err(e) => Err(e),
        Ok(_) => Ok(())
    }
}