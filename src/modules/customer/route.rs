use super::controller::*;
use crate::{common::router::resource_router, db::AppState};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn customer_routes() -> Router<AppState> {
    resource_router(
        "customers",
        get(list_customers),
        post(create_customer),
        get(get_customer),
        put(update_customer),
        delete(delete_customer),
    )
}
