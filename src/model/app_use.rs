use serde::{Serialize, Deserialize};
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
    pub id: i32,
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub app: ViewApp,
    pub public_app: bool
}

#[derive(Serialize)]
pub struct ViewUseOfUser {
    pub id: i32,
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub app: ViewManageApp
}

#[derive(Serialize)]
pub struct ViewUseOfApp {
    pub id: i32,
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub user: ViewManageUser
}

#[derive(Serialize)]
pub struct ViewUse {
    pub id: i32,
    pub last_use: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub user_id: i32,
    pub app_id: i32
}

#[derive(Deserialize)]
pub struct AppVerifyRequest {
    pub app_id: Option<i32>,                //表明使用者app
    pub app_unique_name: Option<String>,    //表明使用者app，这两个二选一
    pub secret: String,     //验证身份的密码

    pub token: Option<String>,       //要查阅的token
    pub user_id: Option<i32>,        //或者用user id
    pub username: Option<String>     //或者用username
}

#[derive(Serialize)]
pub struct AppVerifyResponse {
    pub user_id: i32,
    pub username: String,

    pub is_staff: bool,         //该用户是中央系统的系统管理员
    pub info: Option<String>    //该用户被app附带的附加信息
}

#[derive(Deserialize)]
pub struct InfoUpdateRequest {
    pub app_id: Option<i32>,                //表明使用者app
    pub app_unique_name: Option<String>,    //表明使用者app，这两个二选一
    pub secret: String,     //验证身份的密码

    pub user_id: i32,
    pub info: Option<String>
}