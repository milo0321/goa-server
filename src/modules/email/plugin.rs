use super::route;
use crate::db::AppState;
use crate::modules::email::service::fetch_emails;
use crate::plugin::core::{AppModule, ModuleFactory};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct EmailModule {
    db: Arc<PgPool>,
}

impl EmailModule {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl AppModule for EmailModule {
    fn name(&self) -> &'static str {
        "emails"
    }

    fn route(&self) -> Router<AppState> {
        route::email_routes()
    }

    fn init(&self, _db: Arc<PgPool>) -> Option<JoinHandle<()>> {
        let interval = std::env::var("EMAIL_FETCH_INTERVAL_SECONDS")
            .unwrap_or_else(|_| "300".to_string())
            .parse::<u64>()
            .unwrap_or(300);
        Some(tokio::spawn(async move {
            let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(interval));
            loop {
                ticker.tick().await;
                fetch_emails(&_db).await;
            }
        }))
    }

    fn status(&self) -> String {
        "running".to_string()
    }

    fn shutdown(&self) {
        println!("Shutting down EmailModule");
        // 执行必要的清理操作
    }
}

inventory::submit! {
    ModuleFactory(|db| Box::new(EmailModule::new(db)))
}
