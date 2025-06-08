use super::model::*;
use super::*;
use crate::db::AppState;
use crate::error::ApiError;
use uuid::Uuid;

// 查询所有报价单（分页）
pub async fn list_quotations(
    state: &AppState,
    params: QuotationPaginationParams,
) -> Result<QuotationPaginatedResponse, ApiError> {
    let response = repo::fetch_quotations(&state, params).await?;
    Ok(response)
}

// 创建报价单
pub async fn create_quotation(
    state: &AppState,
    payload: CreateQuotation,
) -> Result<Quotation, ApiError> {
    let created_quotation = repo::insert_quotation(&state, payload).await?;
    Ok(created_quotation)
}

// 获取单个报价单详细信息
pub async fn get_quotation(state: &AppState, quotation_id: Uuid) -> Result<Quotation, ApiError> {
    let quotation = repo::fetch_quotation_by_id(&state, quotation_id).await?;
    Ok(quotation)
}

// 更新报价单
pub async fn update_quotation(
    state: &AppState,
    quotation_id: Uuid,
    payload: UpdateQuotation,
) -> Result<Quotation, ApiError> {
    let quotation = repo::update_quotation(&state, quotation_id, payload).await?;
    Ok(quotation)
}

// 删除报价单
pub async fn delete_quotation(state: &AppState, quotation_id: Uuid) -> Result<(), ApiError> {
    repo::delete_quotation(&state, quotation_id).await?;
    Ok(())
}
