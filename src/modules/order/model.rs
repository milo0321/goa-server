use crate::common::packing::PackingDetail;
use crate::modules::supplier::model::Supplier;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, Type};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Draft,
    Sampling,
    SampleApproved,
    MassProduction,
    ReadyToShip,
    Shipped,
    Completed,
    Cancelled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Draft
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: Uuid,
    pub order_no: String,
    pub order_article: String,
    pub customer_id: Uuid,
    pub customer_order_no: String,
    pub customer_name: String,
    pub currency: String,
    pub payment_terms: String,
    pub delivery_time: DateTime<Utc>,
    pub shipping_method: String,
    pub remarks: Option<String>,
    pub status: OrderStatus,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
    pub order_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub item_no: String,
    pub article: String,
    pub quantity: i32,
    pub unit: String,
    pub unit_price: f64,
    pub vat_rate: f64,
    pub subtotal: f64,
    pub vat_amount: f64,
    pub total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub component_name: String,
    pub component_type: String, // e.g. "Material", "Molding", "Shipping"
    pub quantity: f64,
    pub unit: String,
    pub unit_cost: f64,
    pub total_cost: f64,
    pub supplier_id: Option<Uuid>,
    pub remarks: Option<String>,
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
    pub order_no: String,
    pub order_article: String,
    pub customer_id: Uuid,
    pub customer_order_no: String,
    pub currency: String,
    pub payment_terms: String,
    pub delivery_time: DateTime<Utc>,
    pub shipping_method: Option<String>,
    pub remarks: Option<String>,
    pub status: OrderStatus,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
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
