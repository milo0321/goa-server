use super::route;
use crate::db::AppState;
use crate::plugin::core::{AppModule, ModuleFactory};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct CustomerModule {
    db: Arc<PgPool>,
}

impl CustomerModule {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl AppModule for CustomerModule {
    fn name(&self) -> &'static str {
        "customer"
    }

    fn route(&self) -> Router<AppState> {
        route::customer_routes()
    }

    fn init(&self, _db: Arc<PgPool>) -> Option<JoinHandle<()>> {
        None
    }

    fn status(&self) -> String {
        "running".to_string()
    }

    fn shutdown(&self) {
        println!("Shutting down CustomerModule");
        // 执行必要的清理操作
    }
}

// 使用工厂函数的方式注册模块
inventory::submit! {
    ModuleFactory(|db| Box::new(CustomerModule::new(db)))
}
