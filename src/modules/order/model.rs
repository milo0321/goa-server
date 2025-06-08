use crate::common::packing::PackingDetail;
use crate::modules::supplier::model::Supplier;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub name: String,
    pub quantity: Option<i32>,
    pub unit_price: Option<OrderedFloat<f64>>,
    pub currency: Option<String>,
    pub cost: Option<OrderedFloat<f64>>,
    pub supplier: Option<Supplier>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Uuid,
    pub customer_order_number: String,
    pub quotation_id: Option<Uuid>,
    pub article: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub currency: Option<String>,
    pub costs: Option<Json<Vec<Cost>>>,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
    pub order_date: DateTime<Utc>, // 新增字段
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for Order {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrder {
    pub order_number: String,
    pub customer_id: Uuid,
    pub customer_order_number: String,
    pub quotation_id: Option<Uuid>,
    pub article: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub currency: Option<String>,
    pub costs: Option<Json<Vec<Cost>>>,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub order_date: DateTime<Utc>,
}

impl IntoResponse for CreateOrder {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

pub type UpdateOrder = CreateOrder;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub customer_id: Option<Uuid>,    // Example filter by supplier
    pub product_name: Option<String>, // Example filter by product
    pub search_query: Option<String>, // Example search query for specific quotations
}
