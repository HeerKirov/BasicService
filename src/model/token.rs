use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

pub struct Token {
    pub key: String,
    pub user_id: i32,
    pub expire_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct CreateToken {
    pub username: String,
    pub password: String,
    pub effective: Option<i64>,
    pub effective_unlimit: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveToken {
    pub key: String,
    pub username: String,

    pub expire_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct UpdateToken {
    pub effective: i64
}