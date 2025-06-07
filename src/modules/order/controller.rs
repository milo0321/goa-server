use crate::{common::pagination::PaginatedResponse, db::AppState, error::ApiError};
use axum::extract::State;
use axum::extract::{Json, Path, Query};
use axum::http::StatusCode;
use uuid::Uuid;

use super::model::*;
use super::*;

// 查询所有报价单（分页）
pub async fn list_orders(
    State(state): State<AppState>,
    Query(params): Query<OrderPaginationParams>,
) -> Result<Json<PaginatedResponse<Order>>, ApiError> {
    let response = service::list_orders(&state, params).await?;
    Ok(Json(response))
}

// 创建报价单
pub async fn create_order(
    State(state): State<AppState>,
    Json(create_order): Json<CreateOrder>,
) -> Result<Json<Order>, ApiError> {
    let created_order = service::create_order(&state, create_order).await?;
    Ok(Json(created_order))
}

// 获取单个报价单详细信息
pub async fn get_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<Order>, ApiError> {
    let order = service::get_order(&state, order_id).await?;
    Ok(Json(order))
}

// 更新报价单
pub async fn update_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
    Json(updated_order): Json<UpdateOrder>,
) -> Result<Json<Order>, ApiError> {
    let updated_order = service::update_order(&state, order_id, updated_order).await?;
    Ok(Json(updated_order))
}

// 删除报价单
pub async fn delete_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_order(&state, order_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
