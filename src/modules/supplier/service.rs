use super::model::*;
use super::repository;
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
    let response = repository::list_suppliers(&state, params).await?;
    Ok(response)
}

pub async fn get_supplier(state: &AppState, id: Uuid) -> Result<Supplier, ApiError> {
    let response = repository::get_supplier(&state, Path(id)).await?;
    Ok(response)
}

pub async fn create_supplier(
    state: &AppState,
    params: CreateSupplier,
) -> Result<Supplier, ApiError> {
    let response: Supplier = repository::create_supplier(&state, params).await?;
    Ok(response)
}

pub async fn update_supplier(
    state: &AppState,
    id: Uuid,
    params: UpdateSupplier,
) -> Result<Supplier, ApiError> {
    let response = repository::update_supplier(&state, id, params).await?;
    Ok(response)
}

pub async fn delete_supplier(state: &AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    repository::delete_supplier(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
