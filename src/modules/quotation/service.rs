use super::model::*;
use super::*;
use crate::db::AppState;
use crate::error::ApiError;
use axum::extract::{Path, State};
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn list_quotations(
    State(state): State<AppState>,
    params: QuotationPaginationParams,
) -> Result<QuotationPaginatedResponse, ApiError> {
    let response = repository::fetch_quotations(State(state), params).await?;
    Ok(response)
}

// 创建报价单
pub async fn create_quotation(
    State(state): State<AppState>,
    payload: CreateQuotation,
) -> Result<Quotation, ApiError> {
    let created_quotation = repository::insert_quotation(State(state), payload).await?;
    Ok(created_quotation)
}

// 获取单个报价单详细信息
pub async fn get_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<Quotation, ApiError> {
    let quotation = repository::fetch_quotation_by_id(State(state), Path(quotation_id)).await?;
    Ok(quotation)
}

// 更新报价单
pub async fn update_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
    payload: UpdateQuotation,
) -> Result<Quotation, ApiError> {
    let quotation = repository::update_quotation(State(state), Path(quotation_id), payload).await?;
    Ok(quotation)
}

// 删除报价单
pub async fn delete_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<(), ApiError> {
    repository::delete_quotation(State(state), Path(quotation_id)).await?;
    Ok(())
}
