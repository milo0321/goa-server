use crate::db::AppState;
use crate::plugin::loader::load_plugins;
use axum::{
    Router,
    body::Body,
    http::{Method, Request},
};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;

pub fn init_routes(state: AppState) -> (Router, Vec<JoinHandle<()>>) {
    let db = Arc::clone(&state.db);
    let (plugin_router, _tasks) = load_plugins(db); // 背景任务后面单独管理

    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_origin(Any)
        .allow_headers(Any);

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
            tracing::info!("Received {} {}", request.method(), request.uri().path())
        });

    (
        plugin_router
            .with_state(state)
            .layer(cors)
            .layer(trace_layer),
        _tasks,
    )
}
