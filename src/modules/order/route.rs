use super::controller::*;
use crate::common::router::resource_router;
use crate::db::AppState;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn order_routes() -> Router<AppState> {
    resource_router(
        "orders",
        get(list_orders),
        post(create_order),
        get(get_order),
        put(update_order),
        delete(delete_order),
    )
}
