use crate::models::pagination::{PaginatedResponse};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use sqlx::{FromRow, Type};
use std::error::Error;
use chrono::DateTime;
use uuid::Uuid;

/// 简化 JSONB 类型的宏
macro_rules! impl_jsonb_for {
    ($t:ty) => {
        impl Type<sqlx::Postgres> for $t {
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("jsonb")
            }
        }

        impl<'q> Encode<'q, sqlx::Postgres> for $t {
            fn encode_by_ref(
                &self,
                buf: &mut PgArgumentBuffer,
            ) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
                let json = serde_json::to_value(self)?;
                <serde_json::Value as Encode<sqlx::Postgres>>::encode(json, buf)
            }
        }

        impl<'r> Decode<'r, sqlx::Postgres> for $t {
            fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync>> {
                let json: serde_json::Value = Decode::decode(value)?;
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ShippingPrice {
    pub method: ShippingMethod,
    pub unit_price: OrderedFloat<f64>,
    pub currency: Option<String>,
    pub terms: Option<String>,
}

/// Quantity-based tier pricing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct QuantityTier {
    pub quantity: i32,
    pub prices: Vec<ShippingPrice>,
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

/// Quotation model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Quotation {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_name: String,
    pub quantity_tiers: Vec<QuantityTier>,
    pub additional_fees: Option<Vec<AdditionalFee>>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

/// Input model for creating quotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuotation {
    pub customer_id: Uuid,
    pub product_name: String,
    pub quantity_tiers: Vec<QuantityTier>,
    pub additional_fees: Option<Vec<AdditionalFee>>,
    pub notes: Option<String>,
    pub status: Option<String>,
}

/// Input model for updating quotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuotation {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_name: String,
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
    pub customer_id: Option<Uuid>,     // Example filter by supplier
    pub product_name: Option<String>,      // Example filter by product
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
