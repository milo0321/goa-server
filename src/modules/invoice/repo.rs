use super::model::*;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    error::ApiError,
    modules::order::model::Order,
};
use sqlx::{PgPool, Postgres};
use sqlx::{Row, Transaction};
use std::sync::Arc;
use uuid::Uuid;

pub struct InvoiceRepo {
    pub db: Arc<PgPool>,
}

impl InvoiceRepo {
    pub async fn list(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<Invoice>, ApiError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);

        // Validate pagination parameters
        if page == 0 || limit == 0 || limit > 100 {
            return Err(ApiError::ParamError("page or limit invalid.".to_string()));
        }

        // Calculate offset
        let offset = (page - 1) * limit;

        // Get total count of accounts
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
            .fetch_one(&*self.db)
            .await
            .map_err(|e| ApiError::DatabaseError(e))?;
        if total == 0 {
            return Ok(PaginatedResponse::empty(page, limit));
        }

        // Fetch paginated accounts
        let accounts = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&*self.db)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(PaginatedResponse {
            data: accounts,
            page,
            limit,
            total: total as u64,
        })
    }

    pub async fn get(&self, id: Uuid) -> Result<InvoiceDetail, ApiError> {
        let invoice = sqlx::query_as::<_, Invoice>("SELECT * FROM invoices WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.db)
            .await
            .map_err(ApiError::DatabaseError)?;

        let rows = sqlx::query(
            r#"
            SELECT
                o.id, o.order_no, o.customer_id, o.customer_order_no, o.article, o.quantity, o.unit_price, o. currency, o.created_at, o.updated_at,
                c.name as customer_name,
                oi.amount
            FROM orders o
            JOIN order_invoice oi ON o.id = oi.order_id
            JOIN customers c ON o.customer_id = c.id
            WHERE oi.invoice_id = $1
            "#
        )
            .bind(id)
            .fetch_all(&*self.db)
            .await
            .map_err(ApiError::DatabaseError)?;

        let orders = rows
            .into_iter()
            .map(|row| InvoiceOrderItem {
                order: Order {
                    id: row.get("id"),
                    order_no: row.get("order_no"),
                    order_article: row.get("order"),
                    customer_id: row.get("customer_id"),
                    customer_order_no: row.get("customer_order_no"),
                    customer_name: row.get("customer_name"),
                    currency: row.get("currency"),
                    payment_terms: row.get("payment_terms"),
                    delivery_time: row.get("delivery_time"),
                    shipping_method: row.get("shipping_method"),
                    remarks: row.get("remarks"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    order_date: row.get("order_date"),
                    packing_details: row.get("packing_details"),
                    status: row.get("status"),
                },
                amount: row.get("amount"),
            })
            .collect();

        Ok(InvoiceDetail { invoice, orders })
    }

    pub async fn create(&self, params: CreateInvoice) -> Result<Invoice, ApiError> {
        let mut tx = self.db.begin().await.map_err(ApiError::DatabaseError)?;

        let invoice_id = Uuid::new_v4();

        let rec = sqlx::query_as::<_, Invoice>(
            "INSERT INTO invoices (id, invoice_no, invoice_type, customer_id, total_amount, currency, status, issue_date, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW()) RETURNING *"
        )
            .bind(Uuid::new_v4())
            .bind(&params.invoice_no)
            .bind(&params.invoice_type)
            .bind(&params.customer_id)
            .bind(&params.total_amount)
            .bind(&params.currency)
            .bind(&params.status)
            .bind(&params.issue_date)
            .fetch_one(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        insert_order_links(&mut tx, invoice_id, &params.order_links).await?;

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(rec)
    }

    pub async fn update(&self, id: Uuid, params: UpdateInvoice) -> Result<Invoice, ApiError> {
        let mut tx = self.db.begin().await.map_err(ApiError::DatabaseError)?;

        let updated = sqlx::query_as::<_, Invoice>(
            "UPDATE invoices SET customer_id = $1, issue_date = $2, total_amount = $3, currency = $4, updated_at = NOW() WHERE id = $5 RETURNING *"
        )
            .bind(&params.customer_id)
            .bind(&params.issue_date)
            .bind(&params.total_amount)
            .bind(&params.currency)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        replace_order_links(&mut tx, id, &params.order_links).await?;

        tx.commit().await.map_err(ApiError::DatabaseError)?;
        Ok(updated)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let mut tx = self.db.begin().await.map_err(ApiError::DatabaseError)?;

        sqlx::query("DELETE FROM order_invoice WHERE invoice_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        sqlx::query("DELETE FROM invoices WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        tx.commit().await.map_err(ApiError::DatabaseError)?;
        Ok(())
    }
}

pub(crate) async fn insert_order_links(
    tx: &mut Transaction<'_, Postgres>,
    invoice_id: Uuid,
    links: &[OrderInvoiceLink],
) -> Result<(), ApiError> {
    for link in links {
        sqlx::query("INSERT INTO order_invoice (order_id, invoice_id, amount) VALUES ($1, $2, $3)")
            .bind(link.order_id)
            .bind(invoice_id)
            .bind(link.amount)
            .execute(&mut **tx) // ✅ 双解引用 + borrow
            .await
            .map_err(ApiError::DatabaseError)?;
    }
    Ok(())
}

pub(crate) async fn replace_order_links(
    tx: &mut Transaction<'_, Postgres>,
    invoice_id: Uuid,
    links: &[OrderInvoiceLink],
) -> Result<(), ApiError> {
    sqlx::query("DELETE FROM order_invoice WHERE invoice_id = $1")
        .bind(invoice_id)
        .execute(&mut **tx) // ✅
        .await
        .map_err(ApiError::DatabaseError)?;

    insert_order_links(tx, invoice_id, links).await
}
