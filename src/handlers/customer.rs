use crate::{
    db::AppState,
    models::customer::{CreateCustomer, Customer, UpdateCustomer},
    models::pagination::{PaginatedResponse, PaginationParams},
};
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

pub async fn list_customers(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Customer>>, StatusCode> {
    // Set default values if not provided
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);

    // Validate pagination parameters
    if page == 0 || limit == 0 || limit > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Calculate offset
    let offset = (page - 1) * limit;

    // Get total count of customers
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch paginated customers
    let customers = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PaginatedResponse {
        data: customers,
        page,
        limit,
        total: total as u64,
    }))
}
pub async fn create_customer(
    State(state): State<AppState>,
    Json(payload): Json<CreateCustomer>,
) -> Result<Json<Customer>, StatusCode> {
    tracing::debug!("Creating customer with payload: {:?}", payload);

    let customer = match sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customers (name, email, phone, company, position, address)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.company)
    .bind(&payload.position)
    .bind(&payload.address)
    .fetch_one(&state.db)
    .await
    {
        Ok(customer) => Ok(Json(customer)),
        Err(sqlx::Error::Database(db_err))
            if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) =>
        {
            // 处理唯一约束冲突
            Err(StatusCode::CONFLICT)
        }
        Err(e) => {
            tracing::error!("Failed to create customer: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    customer
}

pub async fn get_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Customer>, StatusCode> {
    let customer = sqlx::query_as::<_, Customer>(
        r#"
        SELECT * FROM customers WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match customer {
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCustomer>,
) -> Result<Json<Customer>, StatusCode> {
    let customer = sqlx::query_as::<_, Customer>(
        r#"
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
        "#,
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.phone)
    .bind(payload.company)
    .bind(payload.position)
    .bind(payload.address)
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(customer))
}

pub async fn delete_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM customers WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
