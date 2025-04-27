mod customer;
mod quotation;

use crate::db::AppState;
use crate::routes::customer::customer_routes;
use crate::routes::quotation::quotation_routes;
use axum::Router;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn init_routes(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_origin(Any) // 开发环境可以使用 Any，生产环境应限制为具体域名
        .allow_headers(Any);
    Router::new()
        .nest("/api/customers", customer_routes())
        .nest("/api/quotations", quotation_routes())
        .with_state(state)
        .layer(cors)
}
