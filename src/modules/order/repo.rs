use super::model::*;
use crate::{
    common::pagination::PaginatedResponse, db::db_conn, db::AppState, define_repo_delete_fn,
    error::ApiError,
};
use tracing::{debug, error};
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn list_orders(
    state: &AppState,
    params: OrderPaginationParams,
) -> Result<PaginatedResponse<Order>, ApiError> {
    debug!("fetch_orders: {:?}", params);

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

    // 获取总数
    let count_query = format!("SELECT COUNT(*) FROM orders q {}", where_clause);
    debug!(count_query);
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(db_conn(&state))
        .await?;
    if total == 0 {
        return Ok(PaginatedResponse::empty(page, limit));
    }

    // 获取报价单列表
    let query_str = format!(
        "SELECT q.*, c.id as customer_id, c.name as customer_name
        FROM orders q
        LEFT JOIN customers c ON q.customer_id = c.id
        {}
        ORDER BY q.created_at DESC
        LIMIT {} OFFSET {}",
        where_clause, limit, offset
    );

    debug!(query_str);

    let mut query = sqlx::query_as::<_, Order>(&query_str);
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

    let orders = query.fetch_all(db_conn(&state)).await?;

    Ok(PaginatedResponse {
        data: orders,
        page,
        limit,
        total: total as u64,
    })
}

// 获取单个报价单
pub async fn get_order(state: &AppState, order_id: Uuid) -> Result<Order, ApiError> {
    debug!("fetch_order_by_id: {:?}", order_id);

    let query = "SELECT * FROM orders WHERE id = $1";
    let order = sqlx::query_as::<_, Order>(query)
        .bind(order_id)
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("Order not found".to_string()),
            _ => e.into(), // 利用 #[from] SqlxError 自动转为 ApiError::DatabaseError
        })?;

    Ok(order)
}

// 创建报价单
pub async fn insert_order(state: &AppState, create_order: CreateOrder) -> Result<Order, ApiError> {
    let trace_id = Uuid::new_v4(); // 为当前请求生成追踪 ID（可选）
    debug!(%trace_id, ?create_order, "insert_order: start");

    let sql = r#"
        WITH new_row AS (
            INSERT INTO orders (
                order_no,
                order_article,
                customer_id,
                customer_order_no,
                currency,
                payment_terms,
                delivery_time,
                shipping_method,
                packing_details,
                status,
                remarks,
                order_date
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            RETURNING *
        )
        SELECT r.*, c.name AS customer_name
        FROM new_row r
        LEFT JOIN customers c ON r.customer_id = c.id
    "#;

    debug!(%trace_id, "Executing SQL insert_order");

    let query_result = sqlx::query_as::<_, Order>(sql)
        .bind(&create_order.order_no)
        .bind(&create_order.order_article)
        .bind(&create_order.customer_id)
        .bind(&create_order.customer_order_no)
        .bind(&create_order.currency)
        .bind(&create_order.payment_terms)
        .bind(&create_order.delivery_time)
        .bind(&create_order.shipping_method)
        .bind(&create_order.packing_details)
        .bind(create_order.status)
        .bind(&create_order.remarks)
        .bind(&create_order.order_date)
        .fetch_one(db_conn(&state))
        .await;

    match query_result {
        Ok(inserted) => {
            debug!(%trace_id, order_id = %inserted.id, "insert_order: success");
            Ok(inserted)
        }
        Err(e) => {
            error!(
            %trace_id,
            error = %e,
            sql = %sql,
            order_no = %create_order.order_no,
            "insert_order failed"
        );
            Err(ApiError::DatabaseError(e))
        }
    }
}

// 更新报价单
pub async fn update_order(
    state: &AppState,
    order_id: Uuid,
    update_order: UpdateOrder,
) -> Result<Order, ApiError> {
    debug!("update_order {}: {:?}", order_id, update_order);

    let sql = r#"
         WITH updated_row AS (
            UPDATE orders
            SET
                order_no                = $1,
                order_article           = $2,
                customer_id             = $3,
                customer_order_no       = $4,
                currency                = $5,
                payment_terms           = $6,
                delivery_time           = $7,
                shipping_method         = $8,
                packing_details         = $9,
                status                  = $10,
                remarks                 = $11,
                order_date              = COALESCE($12, inquiry_date),
                updated_at              = now()
            WHERE id = $13
            RETURNING *
        )
        SELECT r.*, c.name AS customer_name
        FROM updated_row r
        LEFT JOIN customers c ON r.customer_id = c.id
    "#;

    // 2. 绑定参数并执行
    let updated: Order = sqlx::query_as::<_, Order>(sql)
        .bind(&update_order.order_no) // $1
        .bind(&update_order.order_article) // $2
        .bind(&update_order.customer_id) // $3
        .bind(&update_order.customer_order_no) // $4
        .bind(&update_order.currency) // $5
        .bind(&update_order.payment_terms) // $6
        .bind(&update_order.delivery_time) // $7
        .bind(&update_order.shipping_method) // $8
        .bind(&update_order.packing_details) // $9
        .bind(&update_order.status) // $10
        .bind(&update_order.remarks) // $11
        .bind(&update_order.order_date) // $12
        .bind(order_id) // $13
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| {
            error!("update_order failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e)
        })?;

    // 3. 返回完整 Order（含 customer_id & customer_name）
    Ok(updated)
}

// 删除报价单
define_repo_delete_fn!(delete_order, "orders");
