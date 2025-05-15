// src/models/pagination.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: u64,
}

impl<T> PaginatedResponse<T> {
    pub fn _new(data: Vec<T>, page: u32, limit: u32, total: u64) -> Self {
        Self {
            data,
            page,
            limit,
            total,
        }
    }

    pub fn _total_pages(&self) -> u32 {
        ((self.total as f64) / (self.limit as f64)).ceil() as u32
    }

    pub fn _has_next(&self) -> bool {
        (self.page as u64) * (self.limit as u64) < self.total
    }

    pub fn _has_prev(&self) -> bool {
        self.page > 1
    }
}
