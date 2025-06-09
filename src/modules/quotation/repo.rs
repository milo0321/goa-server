// repositories/repo
use super::model::*;
use crate::{
    common::pagination::PaginatedResponse, db::AppState, db::db_conn, define_repo_delete_fn,
    error::ApiError,
};
use sqlx::types::Json;
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn fetch_quotations(
    state: &AppState,
    params: QuotationPaginationParams,
) -> Result<QuotationPaginatedResponse, ApiError> {
    tracing::debug!("fetch_quotations: {:?}", params);

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
    // 获取总数
    let count_query = format!("SELECT COUNT(*) FROM quotations q {}", where_clause);
    tracing::debug!(count_query);
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(db_conn(&state))
        .await?;
    if total == 0 {
        return Ok(PaginatedResponse::empty(page, limit));
    }

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

    let quotations = query.fetch_all(db_conn(&state)).await?;

    Ok(QuotationPaginatedResponse {
        data: quotations,
        page,
        limit,
        total: total as u64,
    })
}

// 获取单个报价单
pub async fn fetch_quotation_by_id(
    state: &AppState,
    quotation_id: Uuid,
) -> Result<Quotation, ApiError> {
    tracing::debug!("fetch_quotation_by_id: {:?}", quotation_id);

    let query = "SELECT * FROM quotations WHERE id = $1";
    let quotation = sqlx::query_as::<_, Quotation>(query)
        .bind(quotation_id)
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("Quotation not found".to_string()),
            _ => e.into(), // 利用 #[from] SqlxError 自动转为 ApiError::DatabaseError
        })?;

    Ok(quotation)
}

// 创建报价单
pub async fn insert_quotation(
    state: &AppState,
    payload: CreateQuotation,
) -> Result<Quotation, ApiError> {
    tracing::debug!("insert_quotation: {:?}", payload);

    // 1. 用 CTE 插入新行，并在同一个 SQL 里 LEFT JOIN customers
    let sql = r#"
        WITH new_row AS (
            INSERT INTO quotations (
                customer_id,
                article,
                client,
                size,
                material,
                color,
                details,
                branding,
                packing,
                quantity,
                certifications,
                price,
                extra_cost,
                notes,
                status,
                inquiry_date
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )
            RETURNING *
        )
        SELECT r.*, c.name AS customer_name
        FROM new_row r
        LEFT JOIN customers c ON r.customer_id = c.id
    "#;

    // 2. 绑定参数并执行
    let inserted: Quotation = sqlx::query_as::<_, Quotation>(sql)
        .bind(payload.customer_id) // $1
        .bind(&payload.article) // $2
        .bind(&payload.client) // $3
        .bind(&payload.size) // $4
        .bind(&payload.material) // $5
        .bind(&payload.color) // $6
        .bind(&payload.details) // $7
        .bind(&payload.branding) // $8
        .bind(&payload.packing) // $9
        .bind(&payload.quantity) // $10
        .bind(&payload.certifications) // $11
        .bind(&payload.price) // $12
        .bind(&payload.extra_cost) // $13
        .bind(&payload.notes) // $14
        .bind(&payload.status) // $15
        .bind(&payload.inquiry_date) // $16
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| {
            tracing::error!("insert_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e)
        })?;

    // 3. 返回包含 customer_name 的 Quotation
    Ok(inserted)
}

// 更新报价单
pub async fn update_quotation(
    state: &AppState,
    quotation_id: Uuid,
    payload: UpdateQuotation,
) -> Result<Quotation, ApiError> {
    tracing::debug!("update_quotation {}: {:?}", quotation_id, payload);

    let sql = r#"
         WITH updated_row AS (
            UPDATE quotations
            SET
                customer_id     = $1,
                article         = $2,
                client          = $3,
                size            = $4,
                material        = $5,
                color           = $6,
                details         = $7,
                branding        = $8,
                packing         = $9,
                quantity        = $10,
                certifications  = $11,
                price           = $12,
                extra_cost      = $13,
                notes           = $14,
                status          = $15,
                inquiry_date    = COALESCE($16, inquiry_date),
                sample_time     = $17,
                mass_time       = $18,
                quote_prices    = $19,
                additional_fees = $20,
                packing_details = $21,
                updated_at      = now()
            WHERE id = $22
            RETURNING *
        )
        SELECT r.*, c.name AS customer_name
        FROM updated_row r
        LEFT JOIN customers c ON r.customer_id = c.id
    "#;

    // 2. 绑定参数并执行
    let updated: Quotation = sqlx::query_as::<_, Quotation>(sql)
        .bind(payload.customer_id) // $1
        .bind(&payload.article) // $2
        .bind(&payload.client) // $3
        .bind(&payload.size) // $4
        .bind(&payload.material) // $5
        .bind(&payload.color) // $6
        .bind(&payload.details) // $7
        .bind(&payload.branding) // $8
        .bind(&payload.packing) // $9
        .bind(&payload.quantity) // $10
        .bind(&payload.certifications) // $11
        .bind(&payload.price) // $12
        .bind(&payload.extra_cost) // $13
        .bind(&payload.notes) // $14
        .bind(&payload.status) // $15
        .bind(&payload.inquiry_date) // $16
        .bind(payload.sample_time.map(Json)) // $17
        .bind(payload.mass_time.map(Json)) // $18
        .bind(payload.quote_prices.map(Json)) // $19
        .bind(payload.additional_fees.map(Json)) // $20
        .bind(payload.packing_details.map(Json)) // $21
        .bind(quotation_id) // $22
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| {
            tracing::error!("update_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e)
        })?;

    // 3. 返回完整 Quotation（含 customer_id & customer_name）
    Ok(updated)
}

// 删除报价单
define_repo_delete_fn!(delete_quotation, "quotations");
