use crate::models::pagination::PaginatedResponse;
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// Shipping method options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum ShippingMethod {
    #[default]
    Air,
    Ship,
    Express,
}

/// Fee type options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum FeeTypeKind {
    #[default]
    Sampling,
    Mold,
    Certification,
    Other,
}

/// Structured fee type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FeeType {
    pub kind: FeeTypeKind,
    pub detail: Option<String>,
}

/// Shipping price structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuantityPrice {
    pub quantity: i32,
    pub unit_price: OrderedFloat<f64>,
    pub currency: Option<String>,
}

/// Quantity-based tier pricing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct QuotePrice {
    pub method: String,
    pub destination: Option<String>,
    pub terms: Option<String>,
    pub prices: Vec<QuantityPrice>,
}

/// Additional fees (e.g., samples, molds)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalFee {
    pub fee_type: String,
    pub amount: OrderedFloat<f64>,
    pub refundable: Option<bool>,
    pub conditions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackingField {
    pub value: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SizeField {
    pub length: OrderedFloat<f32>,
    pub width: OrderedFloat<f32>,
    pub height: OrderedFloat<f32>,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackingDetail {
    pub inner_pack: PackingField,
    pub outer_pack: PackingField,
    pub carton_size: SizeField,
    pub weight: PackingField,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductionTime {
    pub time_type: String,
    pub from_time: i32,
    pub to_time: Option<i32>,
    pub unit: String,
}

/// Quotation model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Quotation {
    pub id: Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub inquiry_date: DateTime<Utc>, // 新增字段
    pub customer_id: Uuid,
    pub customer_name: String,
    pub article: String,
    pub client: Option<String>,
    pub size: Option<String>,
    pub material: Option<String>,
    pub color: Option<String>,
    pub details: Option<String>,
    pub branding: Option<String>,
    pub packing: Option<String>,
    pub quantity: Option<String>,
    pub certifications: Option<String>,
    pub price: Option<String>,
    pub extra_cost: Option<String>,
    pub sample_time: Option<Json<ProductionTime>>,
    pub mass_time: Option<Json<ProductionTime>>,
    pub quote_prices: Option<Json<Vec<QuotePrice>>>,
    pub additional_fees: Option<Json<Vec<AdditionalFee>>>,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

/// Input model for creating quotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuotation {
    pub customer_id: Uuid,
    pub article: String,
    pub client: Option<String>,
    pub size: Option<String>,
    pub material: Option<String>,
    pub color: Option<String>,
    pub details: Option<String>,
    pub branding: Option<String>,
    pub packing: Option<String>,
    pub quantity: Option<String>,
    pub certifications: Option<String>,
    pub price: Option<String>,
    pub extra_cost: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inquiry_date: Option<DateTime<Utc>>, // 可选字段，前端可不传
}

/// Input model for updating quotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuotation {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub article: String,
    pub client: Option<String>,
    pub size: Option<String>,
    pub material: Option<String>,
    pub color: Option<String>,
    pub details: Option<String>,
    pub branding: Option<String>,
    pub packing: Option<String>,
    pub quantity: Option<String>,
    pub certifications: Option<String>,
    pub price: Option<String>,
    pub extra_cost: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inquiry_date: Option<DateTime<Utc>>, // 可选的更新时间
    pub sample_time: Option<Json<ProductionTime>>,
    pub mass_time: Option<Json<ProductionTime>>,
    pub quote_prices: Option<Json<Vec<QuotePrice>>>,
    pub additional_fees: Option<Json<Vec<AdditionalFee>>>,
    pub packing_details: Option<Json<Vec<PackingDetail>>>,
}

/// Quotation-specific Pagination Params
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotationPaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub customer_id: Option<Uuid>,    // Example filter by supplier
    pub product_name: Option<String>, // Example filter by product
    pub search_query: Option<String>, // Example search query for specific quotations
}

// impl QuotationPaginationParams {
//     pub fn to_pagination_params(&self) -> PaginationParams {
//         PaginationParams {
//             page: self.page,
//             limit: self.limit,
//         }
//     }
// }

/// Paginated Response for Quotation, inherits from PaginatedResponse
pub type QuotationPaginatedResponse = PaginatedResponse<Quotation>;
