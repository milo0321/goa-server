use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
pub struct Customer {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    #[sqlx(default)] // 处理可能的 NULL 值
    pub phone: Option<String>,
    #[sqlx(default)]
    pub company: Option<String>,
    #[sqlx(default)]
    pub position: Option<String>,
    #[sqlx(default)]
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomer {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
}
