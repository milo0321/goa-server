// repositories/repository.rs
use crate::{db::AppState, error::ApiError, models::quotation::*};
use axum::extract::{Path, State};
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn fetch_quotations(
    State(state): State<AppState>,
    params: QuotationPaginationParams,
) -> Result<QuotationPaginatedResponse, ApiError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut conditions = Vec::new();
    // 动态生成占位符序号（如 $1, $2）
    let mut arg_counter = 1;

    if let Some(_) = params.customer_id {
        conditions.push(format!("q.customer_id = ${}", arg_counter));
        arg_counter += 1;
    }

    if let Some(_) = params.product_name {
        conditions.push(format!("q.product_name = ${}", arg_counter));
        arg_counter += 1;
    }

    if let Some(_) = &params.search_query {
        conditions.push(format!("q.notes ILIKE ${}", arg_counter));
        // arg_counter += 1;
    }

    // 构建 where 子句
    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // 获取报价单列表
    // 获取报价单列表
    let query_str = format!(
        "SELECT q.*, c.id as customer_id, c.name as customer_name
        FROM quotations q
        LEFT JOIN customers c ON q.customer_id = c.id
        {}
        ORDER BY q.created_at DESC
        LIMIT {} OFFSET {}",
        where_clause, limit, offset
    );

    tracing::debug!(query_str);

    let mut query = sqlx::query_as::<_, Quotation>(&query_str);
    // 按实际存在的参数顺序绑定（与占位符顺序严格一致）
    if let Some(customer_id) = params.customer_id {
        query = query.bind(customer_id); // 绑定 i64
    }

    if let Some(product_name) = params.product_name {
        query = query.bind(product_name); // 绑定 i64
    }

    if let Some(search_query) = &params.search_query {
        query = query.bind(format!("%{}%", search_query)); // 绑定 String
    }

    // 获取总数
    let count_query = format!("SELECT COUNT(*) FROM quotations q {}", where_clause);
    tracing::debug!(count_query);
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&state.db)
        .await?;

    let mut quotations = Vec::new();
    if total > 0 {
        quotations = query.fetch_all(&state.db).await?;
    }

    Ok(QuotationPaginatedResponse {
        data: quotations,
        page,
        limit,
        total: total as u64,
    })
}
// 创建报价单
pub async fn insert_quotation(
    State(state): State<AppState>,
    quotation: CreateQuotation,
) -> Result<Quotation, ApiError> {
    let query = r#"
        INSERT INTO quotations (customer_id, product_name, quantity_tiers, additional_fees, shipping_prices, notes)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, customer_id, product_name, quantity_tiers, additional_fees, shipping_prices, notes
    "#;

    let new_quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(quotation.customer_id)
        .bind(quotation.product_name)
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
    Path(quotation_id): Path<Uuid>,
) -> Result<Quotation, ApiError> {
    let query = "SELECT * FROM quotations WHERE id = $1";
    let quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(quotation_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("Quotation not found".to_string()),
            _ => e.into(), // 利用 #[from] SqlxError 自动转为 ApiError::DatabaseError
        })?;

    Ok(quotation)
}

// 更新报价单
pub async fn update_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
    updated_quotation: UpdateQuotation,
) -> Result<Quotation, ApiError> {
    let query = r#"
        UPDATE quotations
        SET quantity_tiers = $1, additional_fees = $2, shipping_prices = $3, notes = $4
        WHERE id = $5
        RETURNING id, customer_id, product_name, quantity_tiers, additional_fees, shipping_prices, notes
    "#;

    let quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(updated_quotation.quantity_tiers)
        .bind(updated_quotation.additional_fees)
        .bind(updated_quotation.shipping_prices)
        .bind(updated_quotation.notes)
        .bind(quotation_id)
        .fetch_one(&state.db)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(quotation)
}

// 删除报价单
pub async fn delete_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<(), ApiError> {
    let query = "DELETE FROM quotations WHERE id = $1";
    sqlx::query(query)
        .bind(quotation_id)
        .execute(&state.db)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(())
}
