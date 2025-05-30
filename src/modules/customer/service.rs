use super::model::*;
use super::repository;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn list_customers(
    State(state): State<AppState>,
    params: PaginationParams,
) -> Result<PaginatedResponse<Customer>, ApiError> {
    let response = repository::list_customers(State(state), params).await?;
    Ok(response)
}

pub async fn get_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Customer, ApiError> {
    let response = repository::get_customer(State(state), Path(id)).await?;
    Ok(response)
}

pub async fn create_customer(
    State(state): State<AppState>,
    params: CreateCustomer,
) -> Result<Customer, ApiError> {
    let response: Customer = repository::create_customer(State(state), params).await?;
    Ok(response)
}

pub async fn update_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    params: UpdateCustomer,
) -> Result<Customer, ApiError> {
    let response = repository::update_customer(State(state), Path(id), params).await?;
    Ok(response)
}

pub async fn delete_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    repository::delete_customer(State(state), Path(id)).await?;
    Ok(StatusCode::NO_CONTENT)
}
