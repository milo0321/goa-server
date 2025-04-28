use crate::controllers::quotation::{
    create_quotation, delete_quotation, get_quotation, list_quotations, update_quotation,
};
use crate::db::AppState;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

pub fn quotation_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(list_quotations)
                .post(create_quotation)
                .options(|| async { StatusCode::OK }),
        )
        .route(
            "/{id}",
            get(get_quotation)
                .put(update_quotation)
                .delete(delete_quotation)
                .options(|| async { StatusCode::OK }),
        )
        .layer(TraceLayer::new_for_http())
}
