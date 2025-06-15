use super::route;
use crate::db::AppState;
use crate::plugin::core::{AppModule, ModuleFactory};
use axum::Router;
use tokio::task::JoinHandle;

pub struct InvoiceModule {
    state: AppState,
}

impl InvoiceModule {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl AppModule for InvoiceModule {
    fn name(&self) -> &'static str {
        "invoices"
    }

    fn route(&self) -> Router<AppState> {
        route::invoice_routes()
    }

    fn init(&self) -> Option<JoinHandle<()>> {
        None
    }

    fn status(&self) -> String {
        "running".to_string()
    }

    fn shutdown(&self) {
        println!("Shutting down InvoiceModule");
        // 执行必要的清理操作
    }
}

// 使用工厂函数的方式注册模块
inventory::submit! {
    ModuleFactory(|state| Box::new(InvoiceModule::new(state)))
}
