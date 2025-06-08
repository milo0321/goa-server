use super::model::*;
use super::repo;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use axum::extract::Path;
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn list_suppliers(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<Supplier>, ApiError> {
    let response = repo::list_suppliers(&state, params).await?;
    Ok(response)
}

pub async fn get_supplier(state: &AppState, id: Uuid) -> Result<Supplier, ApiError> {
    let response = repo::get_supplier(&state, Path(id)).await?;
    Ok(response)
}

pub async fn create_supplier(
    state: &AppState,
    params: CreateSupplier,
) -> Result<Supplier, ApiError> {
    let response: Supplier = repo::create_supplier(&state, params).await?;
    Ok(response)
}

pub async fn update_supplier(
    state: &AppState,
    id: Uuid,
    params: UpdateSupplier,
) -> Result<Supplier, ApiError> {
    let response = repo::update_supplier(&state, id, params).await?;
    Ok(response)
}

pub async fn delete_supplier(state: &AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    repo::delete_supplier(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
