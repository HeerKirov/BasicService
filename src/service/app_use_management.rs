use postgres::transaction::Transaction;
use postgres::Error;
use super::super::model::app_use::{ViewUseOfApp, ViewUseOfUser, ViewUse};
use super::super::model::app::ViewManageApp;
use super::super::model::user::{ViewManageUser, CreatePath};

pub fn use_list_of_user(t: &Transaction, user_id: i32) -> Result<Vec<ViewUseOfUser>, Error> {
    match t.query("SELECT sau.id AS id, sau.last_use AS last_use, sau.create_time AS create_time, sau.update_time AS update_time,
            sa.id AS app_id, sa.name AS app_name, sa.unique_name AS app_unique_name, sa.description AS app_description,
            sa.public AS app_public, sa.enable AS app_enable, sa.create_time AS app_create_time, sa.update_time AS app_update_time
            FROM service_app_use sau
            INNER JOIN service_app sa on sau.app_id = sa.id
            WHERE sau.user_id = $1", &[&user_id]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row|ViewUseOfUser {
            id: row.get("id"),
            last_use: row.get("last_use"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time"),
            app: ViewManageApp {
                id: row.get("app_id"),
                unique_name: row.get("app_unique_name"),
                name: row.get("app_name"),
                description: row.get("app_description"),
                public: row.get("app_public"),
                enable: row.get("app_enable"),
                create_time: row.get("app_create_time"),
                update_time: row.get("app_update_time")
            }
        }).collect())
    }
}

pub fn use_list_of_app(t: &Transaction, app_id: i32) -> Result<Vec<ViewUseOfApp>, Error> {
    match t.query("SELECT sau.id AS id, sau.last_use AS last_use, sau.create_time AS create_time, sau.update_time AS update_time,
            su.id AS u_id, su.username AS u_username, su.name AS u_name, su.cover AS u_cover, su.is_staff AS u_is_staff,
            su.last_login AS u_last_login, su.last_login_ip AS u_last_login_ip,
            su.create_time AS u_create_time, su.create_path AS u_create_path, su.enable AS u_enable
            FROM service_app_use sau
            INNER JOIN service_user su ON su.id = sau.user_id
            WHERE sau.app_id = $1", &[&app_id]) {
        Err(e) => Err(e),
        Ok(rows) => Ok(rows.iter().map(|row|ViewUseOfApp {
            id: row.get("id"),
            last_use: row.get("last_use"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time"),
            user: ViewManageUser {
                id: row.get("u_id"),
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

pub fn use_get_by_id(t: &Transaction, use_id: i32) -> Result<Option<ViewUse>, Error> {
    match t.query("SELECT id, last_use, create_time, update_time, user_id, app_id
            FROM service_app_use WHERE id = $1 LIMIT 1", &[&use_id]) {
        Err(e) => Err(e),
        Ok(rows) => if rows.len() > 0 {
            Ok(Some(ViewUse {
                id: rows.get(0).get("id"),
                last_use: rows.get(0).get("last_use"),
                create_time: rows.get(0).get("create_time"),
                update_time: rows.get(0).get("update_time"),
                user_id: rows.get(0).get("user_id"),
                app_id: rows.get(0).get("app_id")
            }))
        }else{
            Ok(None)
        }
    }
}

pub fn use_delete_by_app(t: &Transaction, app_id: i32) -> Result<(), Error> {
    match t.execute("DELETE FROM service_app_use WHERE app_id = $1", &[&app_id]) {
        Err(e) => Err(e),
        Ok(_) => Ok(())
    }
}