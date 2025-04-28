use crate::models::pagination::{PaginatedResponse, PaginationParams};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use sqlx::{FromRow, Type};
use std::error::Error;

/// 简化 JSONB 类型的宏
macro_rules! impl_jsonb_for {
    ($t:ty) => {
        impl Type<sqlx::Postgres> for $t {
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("jsonb")
            }
        }

        impl<'q> Encode<'q, sqlx::Postgres> for $t {
            fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
                let json = serde_json::to_value(self).unwrap();
                <serde_json::Value as Encode<sqlx::Postgres>>::encode(json, buf).unwrap()
            }
        }

        impl<'r> Decode<'r, sqlx::Postgres> for $t {
            fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync>> {
                let json: serde_json::Value =
                    <serde_json::Value as Decode<sqlx::Postgres>>::decode(value)?;
                Ok(serde_json::from_value(json)?)
            }
        }

        impl PgHasArrayType for $t {
            fn array_type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("_jsonb")
            }
        }
    };
}

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
pub struct ShippingPrice {
    pub method: ShippingMethod,
    pub unit_price: f64,
    pub currency: Option<String>,
    pub terms: Option<String>,
}

/// Quantity-based tier pricing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct QuantityTier {
    pub min_quantity: i32,
    pub max_quantity: Option<i32>,
    pub unit_price: OrderedFloat<f64>,
    pub currency: Option<String>,
}

/// Additional fees (e.g., samples, molds)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AdditionalFee {
    pub fee_type: FeeType,
    pub amount: OrderedFloat<f64>,
    pub currency: Option<String>,
    pub description: Option<String>,
}

/// Quotation model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
pub struct Quotation {
    pub id: i32,
    pub product_id: i32,
    pub supplier_id: i32,
    pub quantity_tiers: Vec<QuantityTier>,
    pub additional_fees: Option<Vec<AdditionalFee>>,
    pub shipping_prices: Option<Vec<ShippingPrice>>,
    pub notes: Option<String>,
}

/// Input model for creating quotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateQuotation {
    pub product_id: i32,
    pub supplier_id: i32,
    pub quantity_tiers: Vec<QuantityTier>,
    pub additional_fees: Option<Vec<AdditionalFee>>,
    pub shipping_prices: Option<Vec<ShippingPrice>>,
    pub notes: Option<String>,
}

/// Input model for updating quotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateQuotation {
    pub id: i32,
    pub quantity_tiers: Option<Vec<QuantityTier>>,
    pub additional_fees: Option<Vec<AdditionalFee>>,
    pub shipping_prices: Option<Vec<ShippingPrice>>,
    pub notes: Option<String>,
}

// 应用统一 JSONB 处理
impl_jsonb_for!(QuantityTier);
impl_jsonb_for!(AdditionalFee);
impl_jsonb_for!(ShippingPrice);
impl_jsonb_for!(FeeType);

/// Quotation-specific Pagination Params
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotationPaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub supplier_id: Option<i32>,     // Example filter by supplier
    pub product_id: Option<i32>,      // Example filter by product
    pub search_query: Option<String>, // Example search query for specific quotations
}

impl QuotationPaginationParams {
    pub fn to_pagination_params(&self) -> PaginationParams {
        PaginationParams {
            page: self.page,
            limit: self.limit,
        }
    }
}

/// Paginated Response for Quotation, inherits from PaginatedResponse
pub type QuotationPaginatedResponse = PaginatedResponse<Quotation>;
