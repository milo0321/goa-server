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
pub async fn list_accounts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<PaginatedResponse<EmailAccount>, ApiError> {
    let response = service::list_accounts(state, params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EmailAccount, ApiError> {
    let response = service::get_account(state, id).await?;
    Ok(response)
}

#[debug_handler]
pub async fn create_account(
    State(state): State<AppState>,
    Query(params): Query<CreateEmailAccount>,
) -> Result<EmailAccount, ApiError> {
    let response: EmailAccount = service::create_account(state, params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<UpdateEmailAccount>,
) -> Result<EmailAccount, ApiError> {
    let response = service::update_account(state, id, params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_account(state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn list_messages(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<PaginatedResponse<EmailMessage>, ApiError> {
    let response = service::list_messages(state, params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn get_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EmailMessage, ApiError> {
    let response = service::get_message(state, id).await?;
    Ok(response)
}

#[debug_handler]
pub async fn create_message(
    State(state): State<AppState>,
    Query(params): Query<CreateEmailMessage>,
) -> Result<EmailMessage, ApiError> {
    let response: EmailMessage = service::create_message(state, params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn update_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<UpdateEmailMessage>,
) -> Result<EmailMessage, ApiError> {
    let response = service::update_message(state, id, params).await?;
    Ok(response)
}

#[debug_handler]
pub async fn delete_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_message(state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
