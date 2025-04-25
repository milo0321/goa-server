use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use std::fmt;

use crate::models::customer::Customer;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quotation {
    pub id: uuid::Uuid,
    pub inquiry_date: DateTime<Utc>,
    pub customer_id: uuid::Uuid,

    #[sqlx(flatten)]
    pub customer: Customer,

    pub product_name: String,
    pub quantity: i32,

    #[sqlx(rename = "status")]
    pub status: QuotationStatus,

    #[sqlx(default)]
    pub quoted_price: Option<f64>,

    #[sqlx(default)]
    pub quoted_date: Option<DateTime<Utc>>,

    #[sqlx(default)]
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum QuotationStatus {
    Pending,
    Quoted,
}

impl fmt::Display for QuotationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            QuotationStatus::Pending => "pending",
            QuotationStatus::Quoted => "quoted",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuotation {
    pub inquiry_date: DateTime<Utc>,
    pub customer_id: uuid::Uuid,
    pub product_name: String,
    pub quantity: i32,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateQuotation {
    pub product_name: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<QuotationStatus>,
    pub quoted_price: Option<f64>,
    pub quoted_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}