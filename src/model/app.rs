use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

pub struct App {
    pub id: i32,
    pub unique_name: String,    //app的唯一标示名
    pub name: String,           //名称
    pub description: String,    //描述
    pub secret: String,         //子系统登录本系统时，用来验证的密码

    pub public: bool,           //该app是公开项目，出现在公有应用列表。相应地，标记为false的app存在，但不开放给所有用户。
    pub enable: bool,           //该app可用
    pub deleted: bool,          //删除此app

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>
}

#[derive(Serialize)]
pub struct ViewApp {
    pub id: i32,
    pub name: String,
    pub description: String,

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>
}

#[derive(Serialize)]
pub struct ViewManageApp {
    pub id: i32,
    pub unique_name: String,
    pub name: String,
    pub description: String,

    pub public: bool,
    pub enable: bool,

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>
}

#[derive(Serialize)]
pub struct ViewManageSecret {
    pub secret: String
}

#[derive(Deserialize)]
pub struct CreateApp {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub public: bool
}

#[derive(Deserialize)]
pub struct UpdateApp {
    pub name: String,
    pub description: String,
    pub public: bool,
    pub enable: bool
}