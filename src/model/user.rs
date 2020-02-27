use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub enum CreatePath {
    System,     //由系统创建。只有系统默认账户会通过这个途径创建
    Code,       //使用注册码注册
    Public,     //在开放注册时注册
    Admin       //由系统管理员在后台创建
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub cover: Option<String>,
    pub is_staff: bool,      //此用户拥有管理basic service system的权限

    pub last_login: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,

    pub create_time: DateTime<Utc>,
    pub create_path: CreatePath,

    pub enable: bool,
    pub deleted: bool
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    pub name: String,

    pub code: Option<String>
}

#[derive(Serialize)]
pub struct ViewUser {
    pub username: String,
    pub name: String,
    pub cover: Option<String>,
    pub is_staff: bool,

    pub last_login: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,

    pub create_time: DateTime<Utc>,
    pub create_path: CreatePath
}

#[derive(Serialize)]
pub struct ViewCover {
    pub cover: String
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: String
}

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String
}

#[derive(Serialize)]
pub struct ViewManageUser {
    pub username: String,
    pub name: String,
    pub cover: Option<String>,
    pub is_staff: bool,

    pub last_login: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,

    pub create_time: DateTime<Utc>,
    pub create_path: CreatePath,

    pub enable: bool
}

#[derive(Deserialize)]
pub struct CreateManageUser {
    pub username: String,
    pub password: String,
    pub name: String,
    pub is_staff: bool
}

#[derive(Deserialize)]
pub struct UpdateManageUser {
    pub enable: bool
}

#[derive(Deserialize)]
pub struct UpdateManagePassword {
    pub new_password: String
}

impl CreatePath {
    pub fn to_string(&self) -> String {
        match self {
            Self::System => "System",
            Self::Code => "Code",
            Self::Public => "Public",
            Self::Admin => "Admin"
        }.to_string()
    }
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "System" => Some(Self::System),
            "Code" => Some(Self::Code),
            "Public" => Some(Self::Public),
            "Admin" => Some(Self::Admin),
            _ => None
        }
    }
}