// repositories/repository.rs
use crate::{db::AppState, error::ApiError, models::quotation::*};
use axum::extract::State;

// 查询所有报价单（分页）
pub async fn fetch_quotations(
    State(state): State<AppState>,
    params: QuotationPaginationParams,
) -> Result<QuotationPaginatedResponse, ApiError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut conditions = vec![];
    let mut values: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();

    if let Some(supplier_id) = params.supplier_id {
        conditions.push(format!("q.supplier_id = ${}", supplier_id));
        values.push(Box::new(supplier_id));
    }

    if let Some(product_id) = params.product_id {
        conditions.push(format!("q.product_id = ${}", product_id));
        values.push(Box::new(product_id));
    }

    if let Some(search_query) = params.search_query {
        conditions.push(format!("q.notes ILIKE ${}", search_query));
        values.push(Box::new(format!("%{}%", search_query)));
    }

    // 构建 where 子句
    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // 获取报价单列表
    // 获取报价单列表
    let query = format!(
        "SELECT q.*, c.id as customer_id, c.name as customer_name
        FROM quotations q
        LEFT JOIN customers c ON q.customer_id = c.id
        {}
        ORDER BY q.created_at DESC
        LIMIT ${} OFFSET ${}",
        where_clause,
        limit,
        offset
    );

    let mut query = sqlx::query_as::<_, Quotation>(&query);
    for value in values {
        query = query.bind(value);
    }

    let quotations = query.fetch_all(&state.db).await?;

    // 获取总数
    let count_query = format!("SELECT COUNT(*) FROM quotations q {}", where_clause);
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&state.db)
        .await?;

    Ok((quotations, total))
}
// 创建报价单
pub async fn insert_quotation(
    State(state): State<AppState>,
    quotation: CreateQuotation,
) -> Result<Quotation, ApiError> {
    let query = r#"
        INSERT INTO quotations (product_id, supplier_id, quantity_tiers, additional_fees, shipping_prices, notes)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, product_id, supplier_id, quantity_tiers, additional_fees, shipping_prices, notes
    "#;

    let new_quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(quotation.product_id)
        .bind(quotation.supplier_id)
        .bind(quotation.quantity_tiers)
        .bind(quotation.additional_fees)
        .bind(quotation.shipping_prices)
        .bind(quotation.notes)
        .fetch_one(&state.db)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(new_quotation)
}

// 获取单个报价单
pub async fn fetch_quotation_by_id(
    State(state): State<AppState>,
    quotation_id: i64,
) -> Result<Quotation, ApiError> {
    let query = "SELECT * FROM quotations WHERE id = $1";
    let quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(quotation_id)
        .fetch_one(&state.db)
        .await
        .map_err(ApiError::NotFound)?;

    Ok(quotation)
}

// 更新报价单
pub async fn update_quotation(
    State(state): State<AppState>,
    quotation_id: i64,
    updated_quotation: UpdateQuotation,
) -> Result<Quotation, ApiError> {
    let query = r#"
        UPDATE quotations
        SET quantity_tiers = $1, additional_fees = $2, shipping_prices = $3, notes = $4
        WHERE id = $5
        RETURNING id, product_id, supplier_id, quantity_tiers, additional_fees, shipping_prices, notes
    "#;

    let updated_quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(updated_quotation.quantity_tiers)
        .bind(updated_quotation.additional_fees)
        .bind(updated_quotation.shipping_prices)
        .bind(updated_quotation.notes)
        .bind(quotation_id)
        .fetch_one(&state.db)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(updated_quotation)
}

// 删除报价单
pub async fn delete_quotation(
    State(state): State<AppState>,
    quotation_id: i64,
) -> Result<(), ApiError> {
    let query = "DELETE FROM quotations WHERE id = $1";
    sqlx::query(query)
        .bind(quotation_id)
        .execute(&state.db)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(())
}
