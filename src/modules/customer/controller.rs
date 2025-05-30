use super::model::*;
use super::service;
use crate::error::ApiError;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
#[debug_handler]
pub async fn list_customers(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<PaginatedResponse<Customer>, ApiError> {
    let response = service::list_customers(State(state), params).await?;
    Ok(response)
}
#[debug_handler]
pub async fn get_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Customer, ApiError> {
    let response = service::get_customer(State(state), Path(id)).await?;
    Ok(response)
}

#[debug_handler]
pub async fn create_customer(
    State(state): State<AppState>,
    Query(params): Query<CreateCustomer>,
) -> Result<Customer, ApiError> {
    let response: Customer = service::create_customer(State(state), params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn update_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<UpdateCustomer>,
) -> Result<Customer, ApiError> {
    let response = service::update_customer(State(state), Path(id), params).await?;
    Ok(response)
}
#[debug_handler]
pub async fn delete_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_customer(State(state), Path(id)).await?;
    Ok(StatusCode::NO_CONTENT)
}
