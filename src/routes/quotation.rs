use crate::db::AppState;
use axum::{Router, routing::get};

use crate::handlers::quotation::{
    create_quotation, delete_quotation, get_quotation, list_quotations, update_quotation,
};

pub fn quotation_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_quotations).post(create_quotation))
        .route(
            "/{id}",
            get(get_quotation)
                .put(update_quotation)
                .delete(delete_quotation),
        )
}
