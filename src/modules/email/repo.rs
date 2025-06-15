use super::model::*;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    db::db_conn,
    error::ApiError,
};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn list_accounts(
    state: AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailAccount>, ApiError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Validate pagination parameters
    if page == 0 || limit == 0 || limit > 100 {
        return Err(ApiError::ParamError("page or limit invalid.".to_string()));
    }

    // Calculate offset
    let offset = (page - 1) * limit;

    // Get total count of accounts
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM email_accounts")
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;
    if total == 0 {
        return Ok(PaginatedResponse::empty(page, limit));
    }

    // Fetch paginated accounts
    let accounts = sqlx::query_as::<_, EmailAccount>(
        "SELECT * FROM email_accounts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(db_conn(&state))
    .await
    .map_err(|e| ApiError::DatabaseError(e))?;

    Ok(PaginatedResponse {
        data: accounts,
        page,
        limit,
        total: total as u64,
    })
}

pub async fn get_account(state: &AppState, id: Uuid) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn create_account(
    state: &AppState,
    params: CreateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    tracing::debug!("Creating EmailAccount with payload: {:?}", params);

    let sql = r#"
        INSERT INTO email_accounts (email_address, imap_server, imap_port, username, password, use_ssl)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#;
    let email_account: EmailAccount = sqlx::query_as::<_, EmailAccount>(sql)
        .bind(&params.fields.email_address)
        .bind(&params.fields.imap_server)
        .bind(params.fields.imap_port)
        .bind(&params.fields.username)
        .bind(&params.fields.password)
        .bind(&params.fields.use_ssl)
        .fetch_one(db_conn(&state))
        .await
        .map_err(|e| {
            tracing::error!("insert_quotation failed: {}\nSQL: {}", e, sql);
            ApiError::DatabaseError(e.into())
        })?;

    Ok(email_account)
}

pub async fn update_account(
    state: &AppState,
    id: Uuid,
    params: UpdateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn delete_account(state: &AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn list_messages(
    state: &AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailMessage>, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn get_message(state: &AppState, id: Uuid) -> Result<EmailMessage, ApiError> {
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
    id: Uuid,
    params: UpdateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    Err(ApiError::NotImplemented)
}

pub async fn delete_message(state: &AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    Err(ApiError::NotImplemented)
}
