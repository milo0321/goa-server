use super::model::*;
use crate::common::pagination::{PaginatedResponse, PaginationParams};
use crate::db::AppState;
use crate::error::ApiError;
use crate::modules::email::repository;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::PgPool;
use std::fs;
use uuid::Uuid;

pub async fn list_accounts(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailAccount>, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn get_account(state: &AppState, Path(id): Path<Uuid>) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn create_account(
    state: &AppState,
    params: CreateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn update_account(
    state: &AppState,
    Path(id): Path<Uuid>,
    params: UpdateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn delete_account(
    state: &AppState,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn list_messages(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailMessage>, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn get_message(state: &AppState, Path(id): Path<Uuid>) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn create_message(
    state: &AppState,
    params: CreateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn update_message(
    state: &AppState,
    Path(id): Path<Uuid>,
    params: UpdateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn delete_message(
    state: &AppState,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Err(ApiError::NotImplemented)
}
