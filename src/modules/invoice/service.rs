use super::model::*;
use super::repo::InvoiceRepo;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
    error::ApiError,
};
use uuid::Uuid;

pub struct InvoiceService {
    pub repo: InvoiceRepo,
}

impl InvoiceService {
    pub fn from_state(state: &AppState) -> Self {
        Self {
            repo: InvoiceRepo {
                db: state.db.clone(),
            },
        }
    }
    pub async fn list(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<Invoice>, ApiError> {
        self.repo.list(params).await
    }

    pub async fn get(&self, id: Uuid) -> Result<InvoiceDetail, ApiError> {
        self.repo.get(id).await
    }

    pub async fn create(&self, params: CreateInvoice) -> Result<Invoice, ApiError> {
        self.repo.create(params).await
    }

    pub async fn update(&self, id: Uuid, params: UpdateInvoice) -> Result<Invoice, ApiError> {
        self.repo.update(id, params).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        self.repo.delete(id).await
    }
}
