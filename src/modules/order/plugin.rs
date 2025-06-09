use super::route;
use crate::db::AppState;
use crate::plugin::core::{AppModule, ModuleFactory};
use axum::Router;
use tokio::task::JoinHandle;

pub struct OrderModule {
    state: AppState,
}

impl OrderModule {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl AppModule for OrderModule {
    fn name(&self) -> &'static str {
        "orders"
    }

    fn route(&self) -> Router<AppState> {
        route::order_routes()
    }

    fn init(&self) -> Option<JoinHandle<()>> {
        None
    }

    fn status(&self) -> String {
        "running".to_string()
    }

    fn shutdown(&self) {
        println!("Shutting down OrderModule");
        // 执行必要的清理操作
    }
}

// 使用工厂函数的方式注册模块
inventory::submit! {
    ModuleFactory(|state| Box::new(OrderModule::new(state)))
}
