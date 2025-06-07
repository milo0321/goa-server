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
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub order_date: DateTime<Utc>, // 新增字段
    pub customer_id: Uuid,
    pub customer_name: String,
    pub article: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub currency: Option<String>,
    pub costs: Option<Json<Vec<Cost>>>,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
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
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
}

impl IntoResponse for CreateOrder {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrder {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
}

impl IntoResponse for UpdateOrder {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub customer_id: Option<Uuid>,    // Example filter by supplier
    pub product_name: Option<String>, // Example filter by product
    pub search_query: Option<String>, // Example search query for specific quotations
}
