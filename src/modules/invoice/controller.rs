use super::model::*;
use super::service::InvoiceService;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

pub async fn list_invoices(
    State(state): State<AppState>,
    params: axum::extract::Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Invoice>>, ApiError> {
    let result = InvoiceService::list(&state, params.0).await?;
    Ok(Json(result))
}

pub async fn get_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<InvoiceDetail>, ApiError> {
    let invoice = InvoiceService::get(&state, id).await?;
    Ok(Json(invoice))
}

pub async fn create_invoice(
    State(state): State<AppState>,
    Json(params): Json<CreateInvoice>,
) -> Result<Json<Invoice>, ApiError> {
    let invoice = InvoiceService::create(&state, params).await?;
    Ok(Json(invoice))
}

pub async fn update_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(params): Json<UpdateInvoice>,
) -> Result<Json<Invoice>, ApiError> {
    let invoice = InvoiceService::update(&state, id, params).await?;
    Ok(Json(invoice))
}

pub async fn delete_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), ApiError> {
    InvoiceService::delete(&state, id).await
}
