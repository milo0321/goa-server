use super::controller::*;
use crate::common::router::resource_router;
use crate::db::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn customer_routes() -> Router<AppState> {
    resource_router(
        get(list_customers), // 自动处理提取器
        post(create_customer),
        get(get_customer),
        put(update_customer),
        delete(delete_customer),
    )
}
