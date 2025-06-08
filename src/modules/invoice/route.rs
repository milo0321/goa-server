use super::controller::*;
use crate::{common::router::resource_router, db::AppState};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn invoice_routes() -> Router<AppState> {
    resource_router(
        "invoices",
        get(list_invoices),
        post(create_invoice),
        get(get_invoice),
        put(update_invoice),
        delete(delete_invoice),
    )
}
