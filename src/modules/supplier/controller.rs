use super::model::*;
use super::service;
use crate::error::ApiError;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
pub async fn list_suppliers(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<PaginatedResponse<Supplier>, ApiError> {
    let response = service::list_suppliers(&state, params).await?;
    Ok(response)
}
pub async fn get_supplier(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Supplier, ApiError> {
    let _ = &state;
    let response = service::get_supplier(&state, id).await?;
    Ok(response)
}

pub async fn create_supplier(
    State(state): State<AppState>,
    Json(params): Json<CreateSupplier>,
) -> Result<Supplier, ApiError> {
    let response: Supplier = service::create_supplier(&state, params).await?;
    Ok(response)
}

pub async fn update_supplier(
    State(state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(params): Json<UpdateSupplier>,
) -> Result<Supplier, ApiError> {
    let response = service::update_supplier(&state, _id, params).await?;
    Ok(response)
}
pub async fn delete_supplier(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_supplier(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
