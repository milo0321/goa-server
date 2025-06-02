use crate::common::router::resource_router;
use crate::db::AppState;
use crate::modules::email::controller::*;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};

pub fn email_routes() -> Router<AppState> {
    resource_router(
        get(list_accounts), // 自动处理提取器
        post(create_account),
        get(get_account),
        put(update_account),
        delete(delete_account),
    )
}
