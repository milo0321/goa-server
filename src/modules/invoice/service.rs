use super::model::*;
use super::repo::InvoiceRepo;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use uuid::Uuid;

pub struct InvoiceService;

impl InvoiceService {
    pub async fn list(
        state: &AppState,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<Invoice>, ApiError> {
        InvoiceRepo::list(state, params).await
    }

    pub async fn get(state: &AppState, id: Uuid) -> Result<InvoiceDetail, ApiError> {
        InvoiceRepo::get(state, id).await
    }

    pub async fn create(state: &AppState, params: CreateInvoice) -> Result<Invoice, ApiError> {
        InvoiceRepo::create(state, params).await
    }

    pub async fn update(
        state: &AppState,
        id: Uuid,
        params: UpdateInvoice,
    ) -> Result<Invoice, ApiError> {
        InvoiceRepo::update(state, id, params).await
    }

    pub async fn delete(state: &AppState, id: Uuid) -> Result<(), ApiError> {
        InvoiceRepo::delete(state, id).await
    }
}
