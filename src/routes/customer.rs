use crate::db::AppState;
use crate::handlers::customer::*;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

pub fn customer_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(list_customers)
                .post(create_customer)
                .options(|| async { StatusCode::OK }),
        )
        .route(
            "/{id}",
            get(get_customer)
                .put(update_customer)
                .delete(delete_customer)
                .options(|| async { StatusCode::OK }),
        )
        .layer(TraceLayer::new_for_http())
}
