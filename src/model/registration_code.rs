use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

pub struct RegistrationCode {
    pub id: i32,
    pub code: String,
    pub enable: bool,
    pub deadline: Option<DateTime<Utc>>,
    pub used_time: Option<DateTime<Utc>>,
    pub used_user: Option<String>,
    pub create_time: DateTime<Utc>
}

#[derive(Serialize)]
pub struct ViewRegistrationCode {
    pub id: i32,
    pub code: String,
    pub enable: bool,
    pub deadline: Option<DateTime<Utc>>,
    pub used_time: Option<DateTime<Utc>>,
    pub used_user: Option<String>,
    pub create_time: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct CreateRegistrationCode {
    pub deadline: Option<DateTime<Utc>>
}

#[derive(Deserialize)]
pub struct UpdateRegistrationCode {
    pub deadline: Option<DateTime<Utc>>,
    pub enable: Option<bool>
}