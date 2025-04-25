use crate::{
    db::AppState,
    models::pagination::{PaginatedResponse, PaginationParams},
    models::quotation::{CreateQuotation, Quotation, UpdateQuotation},
};
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

pub async fn list_quotations(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Quotation>>, StatusCode> {
    // 设置默认值
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);

    // 验证分页参数
    if page == 0 || limit == 0 || limit > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 计算偏移量
    let offset = (page - 1) * limit;

    // 获取总报价数
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM quotations")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 获取分页报价数据（带客户信息）
    let quotations = sqlx::query_as::<_, Quotation>(
        r#"
    SELECT 
        q.*,
        c.id as "customer_id",
        c.name as "customer_name",
        c.email as "customer_email",
        c.phone as "customer_phone",
        c.company as "customer_company",
        c.position as "customer_position",
        c.address as "customer_address",
        c.created_at as "customer_created_at",
        c.updated_at as "customer_updated_at"
    FROM quotations q
    JOIN customers c ON q.customer_id = c.id
    ORDER BY q.created_at DESC
    LIMIT $1 OFFSET $2
    "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PaginatedResponse {
        data: quotations,
        page,
        limit,
        total: total as u64,
    }))
}

pub async fn create_quotation(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuotation>,
) -> Result<Json<Quotation>, StatusCode> {
    tracing::debug!("Creating quotation with payload: {:?}", payload);

    let quotation = match sqlx::query_as::<_, Quotation>(
        r#"
        INSERT INTO quotations (
            inquiry_date, customer_id, product_name, 
            quantity, status, notes
        )
        VALUES ($1, $2, $3, $4, 'pending', $5)
        RETURNING *
        "#,
    )
    .bind(payload.inquiry_date)
    .bind(payload.customer_id)
    .bind(payload.product_name)
    .bind(payload.quantity)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
    {
        Ok(quotation) => Ok(Json(quotation)),
        Err(sqlx::Error::Database(db_err))
            if db_err.code() == Some(std::borrow::Cow::Borrowed("23503")) =>
        {
            // 处理外键约束冲突 (customer_id 不存在)
            Err(StatusCode::BAD_REQUEST)
        }
        Err(e) => {
            tracing::error!("Failed to create quotation: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    quotation
}

pub async fn get_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quotation>, StatusCode> {
    let quotation = sqlx::query_as::<_, Quotation>(
        r#"
        SELECT q.*, 
               c.id as "customer_id", 
               c.name, c.email, 
               c.phone, c.company, 
               c.position, c.address,
               c.created_at as "customer_created_at",
               c.updated_at as "customer_updated_at"
        FROM quotations q
        LEFT JOIN customers c ON q.customer_id = c.id
        WHERE q.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match quotation {
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateQuotation>,
) -> Result<Json<Quotation>, StatusCode> {
    let quotation = sqlx::query_as::<_, Quotation>(
        r#"
        UPDATE quotations
        SET
            product_name = COALESCE($1, product_name),
            quantity = COALESCE($2, quantity),
            status = COALESCE($3::text, status),
            quoted_price = COALESCE($4, quoted_price),
            quoted_date = COALESCE($5, quoted_date),
            notes = COALESCE($6, notes),
            updated_at = now()
        WHERE id = $7
        RETURNING *
        "#,
    )
    .bind(payload.product_name)
    .bind(payload.quantity)
    .bind(payload.status.map(|s| s.to_string()))
    .bind(payload.quoted_price)
    .bind(payload.quoted_date)
    .bind(payload.notes)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match quotation {
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM quotations WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
