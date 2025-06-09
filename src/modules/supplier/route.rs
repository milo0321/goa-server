use super::controller::*;
use crate::{common::router::resource_router, db::AppState};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn supplier_routes() -> Router<AppState> {
    resource_router(
        "suppliers",
        get(list_suppliers),
        post(create_supplier),
        get(get_supplier),
        put(update_supplier),
        delete(delete_supplier),
    )
}
