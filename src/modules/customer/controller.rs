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
pub async fn list_customers(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<PaginatedResponse<Customer>, ApiError> {
    let response = service::list_customers(&state, params).await?;
    Ok(response)
}
pub async fn get_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Customer, ApiError> {
    let _ = &state;
    let response = service::get_customer(&state, id).await?;
    Ok(response)
}

pub async fn create_customer(
    State(state): State<AppState>,
    Json(params): Json<CreateCustomer>,
) -> Result<Customer, ApiError> {
    let response: Customer = service::create_customer(&state, params).await?;
    Ok(response)
}

pub async fn update_customer(
    State(state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(params): Json<UpdateCustomer>,
) -> Result<Customer, ApiError> {
    let response = service::update_customer(&state, _id, params).await?;
    Ok(response)
}
pub async fn delete_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_customer(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
