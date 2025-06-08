use super::model::*;
use super::repo;
use crate::{common::pagination::PaginatedResponse, db::AppState, error::ApiError};
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn list_orders(
    state: &AppState,
    params: OrderPaginationParams,
) -> Result<PaginatedResponse<Order>, ApiError> {
    let response = repo::list_orders(&state, params).await?;
    Ok(response)
}

// 创建报价单
pub async fn create_order(state: &AppState, payload: CreateOrder) -> Result<Order, ApiError> {
    let created_order = repo::insert_order(&state, payload).await?;
    Ok(created_order)
}

// 获取单个报价单详细信息
pub async fn get_order(state: &AppState, order_id: Uuid) -> Result<Order, ApiError> {
    let order = repo::get_order(&state, order_id).await?;
    Ok(order)
}

// 更新报价单
pub async fn update_order(
    state: &AppState,
    order_id: Uuid,
    payload: UpdateOrder,
) -> Result<Order, ApiError> {
    let order = repo::update_order(&state, order_id, payload).await?;
    Ok(order)
}

// 删除报价单
pub async fn delete_order(state: &AppState, order_id: Uuid) -> Result<(), ApiError> {
    repo::delete_order(&state, order_id).await?;
    Ok(())
}
