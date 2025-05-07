// repositories/repository.rs
use crate::{db::AppState, error::ApiError, models::quotation::*};
use axum::extract::{Path, State};
use sqlx::types::Json;
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn fetch_quotations(
    State(state): State<AppState>,
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

// 获取单个报价单
pub async fn fetch_quotation_by_id(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<Quotation, ApiError> {
    tracing::debug!("fetch_quotation_by_id: {:?}", quotation_id);

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

// 创建报价单
pub async fn insert_quotation(
    State(state): State<AppState>,
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
                notes,
                quantity_tiers,
                additional_fees,
                status,
                inquiry_date
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $0, $10, $11, $12, $13::jsonb, $14::jsonb, $15, $16
            )
            RETURNING *
        )
        SELECT
            nr.id               AS id,
            nr.customer_id      AS customer_id,
            c.name              AS customer_name,
            nr.article          AS article,
            nr.client           AS client,
            nr.size             AS size,
            nr.material         AS material,
            nr.color            AS color,
            nr.details          AS details,
            nr.branding         AS branding,
            nr.packing          AS packing,
            nr.quantity         AS quantity,
            nr.certifications   AS certifications,
            nr.notes            AS notes,
            nr.quantity_tiers   AS quantity_tiers,
            nr.additional_fees  AS additional_fees,
            nr.status           AS status,
            nr.inquiry_date     AS inquiry_date
        FROM new_row nr
        LEFT JOIN customers c
          ON nr.customer_id = c.id
    "#;

    // 2. 绑定参数并执行
    let inserted: Quotation = sqlx::query_as::<_, Quotation>(sql)
        .bind(payload.customer_id)              // $1
        .bind(&payload.article)                 // $2
        .bind(&payload.client)                  // $3
        .bind(&payload.size)                    // $4
        .bind(&payload.material)                // $5
        .bind(&payload.color)                   // $6
        .bind(&payload.details)                 // $7
        .bind(&payload.branding)                // $8
        .bind(&payload.packing)                 // $9
        .bind(&payload.quantity)                // $10
        .bind(&payload.certifications)          // $11
        .bind(&payload.notes)                   // $12
        .bind(Json(payload.quantity_tiers))     // $13
        .bind(Json(payload.additional_fees))    // $14
        .bind(&payload.status)                  // $15
        .bind(&payload.inquiry_date)            // $16
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("insert_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e.into())
        })?;

    // 3. 返回包含 customer_name 的 Quotation
    Ok(inserted)
}

// 更新报价单
pub async fn update_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
    payload: UpdateQuotation,
) -> Result<Quotation, ApiError> {
    tracing::debug!("update_quotation {}: {:?}", quotation_id, payload);

    let sql = r#"
         WITH updated_row AS (
            UPDATE quotations
            SET
                customer_id    = $1,
                article   = $2,
                quantity_tiers = $3::jsonb,
                additional_fees= $4::jsonb,
                notes          = $5,
                status         = $6,
                inquiry_date   = COALESCE($7, inquiry_date)
            WHERE id = $8
            RETURNING *
        )
        SELECT
            ur.id               AS id,
            ur.customer_id      AS customer_id,
            c.name              AS customer_name,
            ur.article          AS article,
            ur.quantity_tiers   AS quantity_tiers,
            ur.additional_fees  AS additional_fees,
            ur.notes            AS notes,
            ur.status           AS status,
            ur.inquiry_date     AS inquiry_date
        FROM updated_row ur
        LEFT JOIN customers c
          ON ur.customer_id = c.id
    "#;

    // 2. 绑定参数并执行
    let updated: Quotation = sqlx::query_as::<_, Quotation>(sql)
        .bind(payload.customer_id) // $1
        .bind(&payload.article) // $2
        .bind(Json(payload.quantity_tiers)) // $3
        .bind(Json(payload.additional_fees)) // $4
        .bind(&payload.notes) // $5
        .bind(&payload.status) // $6
        .bind(&payload.inquiry_date) // $7
        .bind(quotation_id) // $8
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("update_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e)
        })?;

    // 3. 返回完整 Quotation（含 customer_id & customer_name）
    Ok(updated)
}

// 删除报价单
pub async fn delete_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<(), ApiError> {
    tracing::debug!("delete_quotation: {:?}", quotation_id);
    let query = "DELETE FROM quotations WHERE id = $1";
    sqlx::query(query)
        .bind(quotation_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("delete_quotation failed: {}\nSQL: {}", e, query);
            ApiError::DatabaseError(e)
        })?;

    Ok(())
}
