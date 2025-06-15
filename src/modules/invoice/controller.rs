use super::model::*;
use super::service::InvoiceService;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

pub async fn list_invoices(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Invoice>>, ApiError> {
    let service = InvoiceService::from_state(&state);
    let result = service.list(params).await?;

    Ok(Json(result))
}

pub async fn get_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<InvoiceDetail>, ApiError> {
    let service = InvoiceService::from_state(&state);
    let invoice = service.get(id).await?;

    Ok(Json(invoice))
}

pub async fn create_invoice(
    State(state): State<AppState>,
    Json(params): Json<CreateInvoice>,
) -> Result<Json<Invoice>, ApiError> {
    let service = InvoiceService::from_state(&state);
    let invoice = service.create(params).await?;

    Ok(Json(invoice))
}

pub async fn update_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(params): Json<UpdateInvoice>,
) -> Result<Json<Invoice>, ApiError> {
    let service = InvoiceService::from_state(&state);
    let invoice = service.update(id, params).await?;

    Ok(Json(invoice))
}

pub async fn delete_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), ApiError> {
    let service = InvoiceService::from_state(&state);
    service.delete(id).await
}
