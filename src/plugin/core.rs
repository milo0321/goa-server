use crate::db::AppState;
use axum::Router;
use inventory;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// 所有模块都要实现这个 trait 来暴露自己的功能
pub trait AppModule: Send + Sync {
    fn name(&self) -> &'static str;
    fn route(&self) -> Router<AppState>;
    fn init(&self, _db: Arc<PgPool>) -> Option<JoinHandle<()>> {
        None
    }
    fn status(&self) -> String {
        "unknown".to_string()
    }
    fn shutdown(&self) {
        // 默认实现为空
    }
}

/// 注册项，用于延迟构造模块对象
pub struct ModuleFactory(pub fn(Arc<PgPool>) -> Box<dyn AppModule + Send + Sync>);

// 使 trait 对象能被 inventory 收集
inventory::collect!(ModuleFactory);
