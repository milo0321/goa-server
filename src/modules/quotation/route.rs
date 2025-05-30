use super::controller::*;
use crate::common::router::resource_router;
use crate::db::AppState;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn quotation_routes() -> Router<AppState> {
    resource_router(
        get(list_quotations), // 自动处理提取器
        post(create_quotation),
        get(get_quotation),
        put(update_quotation),
        delete(delete_quotation),
    )
}
