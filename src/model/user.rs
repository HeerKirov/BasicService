use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CreatePath {
    System,     //由系统创建。只有系统默认账户会通过这个途径创建
    Code,       //使用注册码注册
    Public,     //在开放注册时注册
    Admin       //由系统管理员在后台创建
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

pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub name: String,
    pub cover: String,
    pub is_staff: bool,      //此用户拥有管理certificatin service system的权限

    pub last_login: i64,
    pub last_login_ip: String,

    pub create_time: i64,
    pub create_path: CreatePath,

    pub enable: bool,
    pub deleted: bool
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    pub name: String,

    pub key: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ViewUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub cover: String,
    pub is_staff: bool,

    pub last_login: i64,
    pub last_login_ip: String,

    pub create_time: i64,
    pub create_path: CreatePath
}