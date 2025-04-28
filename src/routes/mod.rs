mod customer;
mod quotation;

use crate::{
    db::AppState,
    routes::{
        customer::customer_routes,
        quotation::quotation_routes,
    },
};
use axum::{
    Router,
    body::Body,
    http:: {
        Method,
        Request
    }
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;

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

    // 添加请求追踪中间件
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            tracing::span!(
                Level::INFO,
                "request",
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version()
            )
        })
        .on_request(|request: &Request<Body>, _span: &tracing::Span| {
            tracing::info!(
                "Received {} {}",
                request.method(),
                request.uri().path()
            )
        });

    Router::new()
        .nest("/api/customers", customer_routes())
        .nest("/api/quotations", quotation_routes())
        .with_state(state)
        .layer(cors)
        .layer(trace_layer) // 添加在路由最外层
}
