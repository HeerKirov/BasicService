use serde::{Serialize};
use chrono::prelude::{DateTime, Utc};
use super::app::{ViewApp, ViewManageApp};
use super::user::ViewManageUser;

pub struct AppUse {
    pub id: i32,
    pub user_id: i32,       //链接的用户id
    pub app_id: i32,        //链接的app id

    pub info: Option<String>,               //app为该用户附加的额外设置/身份等信息
    pub last_use: Option<DateTime<Utc>>,    //该用户上一次在此app做验证的时间

    pub create_time: Option<DateTime<Utc>>, //该use case创建的时间
    pub update_time: Option<DateTime<Utc>>  //该use case上次被更新内容的时间
}

#[derive(Serialize)]
pub struct ViewAppUse {
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub app: ViewApp,
    pub public_app: bool
}

#[derive(Serialize)]
pub struct ViewUseOfUser {
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub app: ViewManageApp
}

#[derive(Serialize)]
pub struct ViewUseOfApp {
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub user: ViewManageUser
}

#[derive(Serialize)]
pub struct ViewUse {
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub username: String,
    pub app_id: String
}