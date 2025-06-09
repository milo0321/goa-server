use super::model::*;
use super::repo;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn list_customers(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<Customer>, ApiError> {
    let response = repo::list_customers(&state, params).await?;
    Ok(response)
}

pub async fn get_customer(state: &AppState, id: Uuid) -> Result<Customer, ApiError> {
    let response = repo::get_customer(&state, id).await?;
    Ok(response)
}

pub async fn create_customer(
    state: &AppState,
    params: CreateCustomer,
) -> Result<Customer, ApiError> {
    let response: Customer = repo::create_customer(&state, params).await?;
    Ok(response)
}

pub async fn update_customer(
    state: &AppState,
    id: Uuid,
    params: UpdateCustomer,
) -> Result<Customer, ApiError> {
    let response = repo::update_customer(&state, id, params).await?;
    Ok(response)
}

pub async fn delete_customer(state: &AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    repo::delete_customer(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
