use super::model::*;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    db::db_conn,
    error::ApiError,
    modules::order::model::Order,
};
use sqlx::Row;
use uuid::Uuid;

pub struct InvoiceRepo;

impl InvoiceRepo {
    pub async fn list(
        state: &AppState,
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
            .fetch_one(db_conn(&state))
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
        .fetch_all(db_conn(&state))
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(PaginatedResponse {
            data: accounts,
            page,
            limit,
            total: total as u64,
        })
    }

    pub async fn get(state: &AppState, id: Uuid) -> Result<InvoiceDetail, ApiError> {
        let invoice = sqlx::query_as::<_, Invoice>("SELECT * FROM invoices WHERE id = $1")
            .bind(id)
            .fetch_one(db_conn(&state))
            .await
            .map_err(ApiError::DatabaseError)?;

        let rows = sqlx::query(
            r#"
            SELECT
                o.id, o.order_number, o.customer_id, o.customer_order_number, o.article, o.quantity, o.unit_price, o. currency, o.created_at, o.updated_at,
                c.name as customer_name,
                oi.amount
            FROM orders o
            JOIN order_invoice oi ON o.id = oi.order_id
            JOIN customers c ON o.customer_id = c.id
            WHERE oi.invoice_id = $1
            "#
        )
            .bind(id)
            .fetch_all(db_conn(&state))
            .await
            .map_err(ApiError::DatabaseError)?;

        let orders = rows
            .into_iter()
            .map(|row| {
                InvoiceOrderItem {
                    order: Order {
                        id: row.get("id"),
                        order_number: row.get("order_number"),
                        customer_id: row.get("customer_id"),
                        customer_order_number: row.get("customer_order_number"),
                        quotation_id: None,
                        article: row.get("article"),
                        quantity: row.get("quantity"),
                        unit_price: row.get("unit_price"),
                        currency: row.get("currency"),
                        costs: None,
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                        // 其他字段根据你的 Order 模型定义补充
                        order_date: Default::default(),
                        packing_details: None,
                    },
                    amount: row.get("amount"),
                }
            })
            .collect();

        Ok(InvoiceDetail { invoice, orders })
    }

    pub async fn create(state: &AppState, params: CreateInvoice) -> Result<Invoice, ApiError> {
        let mut tx = db_conn(&state)
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        let invoice_id = Uuid::new_v4();

        let rec = sqlx::query_as::<_, Invoice>(
            "INSERT INTO invoices (id, invoice_number, invoice_type, customer_id, total_amount, currency, status, issue_date, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW()) RETURNING *"
        )
            .bind(Uuid::new_v4())
            .bind(&params.invoice_number)
            .bind(&params.invoice_type)
            .bind(&params.customer_id)
            .bind(&params.total_amount)
            .bind(&params.currency)
            .bind(&params.status)
            .bind(&params.issue_date)
            .fetch_one(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        for link in &params.order_links {
            sqlx::query(
                "INSERT INTO order_invoice (order_id, invoice_id, amount) VALUES ($1, $2, $3)",
            )
            .bind(link.order_id)
            .bind(invoice_id)
            .bind(link.amount)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(rec)
    }

    pub async fn update(
        state: &AppState,
        id: Uuid,
        params: UpdateInvoice,
    ) -> Result<Invoice, ApiError> {
        let mut tx = db_conn(&state)
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

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

        sqlx::query("DELETE FROM order_invoice WHERE invoice_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        for link in &params.order_links {
            sqlx::query(
                "INSERT INTO order_invoice (order_id, invoice_id, amount) VALUES ($1, $2, $3)",
            )
            .bind(link.order_id)
            .bind(id)
            .bind(link.amount)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;
        Ok(updated)
    }

    pub async fn delete(state: &AppState, id: Uuid) -> Result<(), ApiError> {
        let mut tx = db_conn(&state)
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

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
