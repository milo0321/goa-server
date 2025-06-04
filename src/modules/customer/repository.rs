use super::model::{CreateCustomer, Customer, UpdateCustomer};
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    db::db_conn,
    error::ApiError,
};
use axum::extract::Path;
use uuid::Uuid;

pub async fn list_customers(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<Customer>, ApiError> {
    // Set default values if not provided
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Validate pagination parameters
    if page == 0 || limit == 0 || limit > 100 {
        return Err(ApiError::ParamError("page or limit invalid.".to_string()));
    }

    // Calculate offset
    let offset = (page - 1) * limit;

    // Get total count of customers
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

    // Fetch paginated customers
    let customers = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(db_conn(&state))
    .await
    .map_err(|e| ApiError::DatabaseError(e))?;

    Ok(PaginatedResponse {
        data: customers,
        page,
        limit,
        total: total as u64,
    })
}

pub async fn get_customer(state: &AppState, Path(id): Path<Uuid>) -> Result<Customer, ApiError> {
    let sql = r#"
        SELECT * FROM customers WHERE id = $1
        "#;
    let customer = sqlx::query_as::<_, Customer>(sql)
        .bind(id)
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound(format!("Quotation:{} not found", id)),
            _ => e.into(), // 利用 #[from] SqlxError 自动转为 ApiError::DatabaseError
        })?;

    Ok(customer)
}

pub async fn create_customer(
    state: &AppState,
    params: CreateCustomer,
) -> Result<Customer, ApiError> {
    tracing::debug!("Creating customer with payload: {:?}", params);

    let sql = r#"
        INSERT INTO customers (name, email, phone, company, position, address)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#;
    let customer: Customer = sqlx::query_as::<_, Customer>(sql)
        .bind(&params.name)
        .bind(&params.email)
        .bind(&params.phone)
        .bind(&params.company)
        .bind(&params.position)
        .bind(&params.address)
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| {
            tracing::error!("insert_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e.into())
        })?;

    Ok(customer)
}

pub async fn update_customer(
    state: &AppState,
    id: Uuid,
    params: UpdateCustomer,
) -> Result<Customer, ApiError> {
    let sql = r#"
        UPDATE customers
        SET
            name = COALESCE($1, name),
            email = COALESCE($2, email),
            phone = COALESCE($3, phone),
            company = COALESCE($4, company),
            position = COALESCE($5, position),
            address = COALESCE($6, address),
            updated_at = now()
        WHERE id = $7
        RETURNING *
        "#;
    let customer = sqlx::query_as::<_, Customer>(sql)
        .bind(params.name)
        .bind(params.email)
        .bind(params.phone)
        .bind(params.company)
        .bind(params.position)
        .bind(params.address)
        .bind(id)
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| {
            tracing::error!("update_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e)
        })?;

    Ok(customer)
}

pub async fn delete_customer(state: &AppState, id: Uuid) -> Result<(), ApiError> {
    let sql = "DELETE FROM customers WHERE id = $1";
    sqlx::query(sql)
        .bind(id)
        .execute(db_conn(&state))
        .await
        .map_err(|e| {
            tracing::error!("delete_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e)
        })?;

    Ok(())
}
