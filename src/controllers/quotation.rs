// controllers/quotation_controller.rs
use crate::db::AppState;
use crate::models::quotation::*;
use crate::{error::ApiError, services::quotation};
use axum::extract::State;
use axum::{
    extract::{Json, Path, Query},
};
use axum::http::StatusCode;
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn list_quotations(
    State(state): State<AppState>,
    Query(params): Query<QuotationPaginationParams>,
) -> Result<Json<QuotationPaginatedResponse>, ApiError> {
    let response = quotation::list_quotations(State(state), params).await?;
    Ok(Json(response))
}

// 创建报价单
pub async fn create_quotation(
    State(state): State<AppState>,
    Json(create_quotation): Json<CreateQuotation>,
) -> Result<Json<Quotation>, ApiError> {
    let created_quotation = quotation::create_quotation(State(state), create_quotation).await?;
    Ok(Json(created_quotation))
}

// 获取单个报价单详细信息
pub async fn get_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<Json<Quotation>, ApiError> {
    let quotation = quotation::get_quotation(State(state), Path(quotation_id)).await?;
    Ok(Json(quotation))
}

// 更新报价单
pub async fn update_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
    Json(updated_quotation): Json<UpdateQuotation>,
) -> Result<Json<Quotation>, ApiError> {
    let updated_quotation =
        quotation::update_quotation(State(state), Path(quotation_id), updated_quotation).await?;
    Ok(Json(updated_quotation))
}

// 删除报价单
pub async fn delete_quotation(
    State(state): State<AppState>,
    Path(quotation_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    quotation::delete_quotation(State(state), Path(quotation_id)).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

