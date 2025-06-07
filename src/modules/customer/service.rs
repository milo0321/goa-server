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

pub async fn list_customers(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<Customer>, ApiError> {
    let response = repository::list_customers(&state, params).await?;
    Ok(response)
}

pub async fn get_customer(state: &AppState, id: Uuid) -> Result<Customer, ApiError> {
    let response = repository::get_customer(&state, Path(id)).await?;
    Ok(response)
}

pub async fn create_customer(
    state: &AppState,
    params: CreateCustomer,
) -> Result<Customer, ApiError> {
    let response: Customer = repository::create_customer(&state, params).await?;
    Ok(response)
}

pub async fn update_customer(
    state: &AppState,
    id: Uuid,
    params: UpdateCustomer,
) -> Result<Customer, ApiError> {
    let response = repository::update_customer(&state, id, params).await?;
    Ok(response)
}

pub async fn delete_customer(state: &AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    repository::delete_customer(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
