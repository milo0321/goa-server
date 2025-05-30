use super::model::*;
use super::service;
use crate::error::ApiError;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
};
use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    debug_handler,
};
use uuid::Uuid;

#[debug_handler]
pub async fn list_accounts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<PaginatedResponse<EmailAccount>, ApiError> {
    let response = service::list_accounts(State(state), params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EmailAccount, ApiError> {
    let response = service::get_account(State(state), Path(id)).await?;
    Ok(response)
}

#[debug_handler]
pub async fn create_account(
    State(state): State<AppState>,
    Query(params): Query<CreateEmailAccount>,
) -> Result<EmailAccount, ApiError> {
    let response: EmailAccount = service::create_account(State(state), params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<UpdateEmailAccount>,
) -> Result<EmailAccount, ApiError> {
    let response = service::update_account(State(state), Path(id), params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_account(State(state), Path(id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_messages(
    State(state): State<AppState>,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailMessage>, ApiError> {
    let response = service::list_messages(State(state), params).await?;
    Ok(response)
}

pub async fn get_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EmailMessage, ApiError> {
    let response = service::get_message(State(state), Path(id)).await?;
    Ok(response)
}

pub async fn create_message(
    State(state): State<AppState>,
    params: CreateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    let response: EmailMessage = service::create_message(State(state), params).await?;
    Ok(response)
}

pub async fn update_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    params: UpdateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    let response = service::update_message(State(state), Path(id), params).await?;
    Ok(response)
}

pub async fn delete_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_message(State(state), Path(id)).await?;
    Ok(StatusCode::NO_CONTENT)
}
