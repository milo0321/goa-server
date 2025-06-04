use super::route;
use crate::db::AppState;
use crate::plugin::core::{AppModule, ModuleFactory};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct QuotationModule {
    db: Arc<PgPool>,
}

impl QuotationModule {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl AppModule for QuotationModule {
    fn name(&self) -> &'static str {
        "quotations"
    }

    fn route(&self) -> Router<AppState> {
        route::quotation_routes()
    }

    fn init(&self, _db: Arc<PgPool>) -> Option<JoinHandle<()>> {
        None
    }

    fn status(&self) -> String {
        "running".to_string()
    }

    fn shutdown(&self) {
        println!("Shutting down QuotationModule");
        // 执行必要的清理操作
    }
}

// 使用工厂函数的方式注册模块
inventory::submit! {
    ModuleFactory(|db| Box::new(QuotationModule::new(db)))
}
