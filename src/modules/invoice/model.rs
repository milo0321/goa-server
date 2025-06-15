use crate::modules::order::model::Order;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow, Type)]
pub struct Invoice {
    pub id: Uuid,
    pub invoice_no: String,
    pub invoice_type: String,
    pub customer_id: String,
    pub total_amount: f64,
    pub currency: String,
    pub status: String,
    pub issue_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoice {
    pub invoice_no: String,
    pub invoice_type: String,
    pub customer_id: String,
    pub total_amount: f64,
    pub currency: String,
    pub status: String,
    pub issue_date: DateTime<Utc>,
    pub order_links: Vec<OrderInvoiceLink>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Type)]
pub struct OrderInvoiceLink {
    pub order_id: Uuid,
    pub amount: f64,
}

pub type UpdateInvoice = CreateInvoice;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InvoiceOrderItem {
    pub order: Order, // 完整订单信息
    pub amount: f64,  // 在该 invoice 中对应金额
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InvoiceDetail {
    pub invoice: Invoice,
    pub orders: Vec<InvoiceOrderItem>,
}
