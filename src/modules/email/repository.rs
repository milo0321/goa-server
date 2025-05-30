use std::fs;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use crate::common::pagination::{PaginatedResponse, PaginationParams};
use crate::db::AppState;
use crate::error::ApiError;
use crate::modules::email::repository;
use super::model::*;

pub async fn list_accounts(
    State(state): State<AppState>,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailAccount>, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn create_account(
    State(state): State<AppState>,
    params: CreateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    params: UpdateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn list_messages(
    State(state): State<AppState>,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailMessage>, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn get_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn create_message(
    State(state): State<AppState>,
    params: CreateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn update_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    params: UpdateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn delete_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Err(ApiError::NotImplemented)
}
