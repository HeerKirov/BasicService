use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct AppVerifyRequest {
    pub secret: String,     //验证身份的密码
    pub token: String       //要查阅的token
}

#[derive(Serialize)]
pub struct AppVerifyResponse {
    pub username: String
}

#[derive(Deserialize)]
pub struct GetInfoRequest {
    pub secret: String,
    pub username: String
}

#[derive(Deserialize)]
pub struct UpdateInfoRequest {
    pub secret: String,
    pub username: String,
    pub info: Option<String>
}

#[derive(Serialize)]
pub struct InfoResponse {
    pub username: String,
    pub name: String,
    pub is_staff: bool,
    pub info: Option<String>
}