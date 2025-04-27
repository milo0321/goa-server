use db::init_db;
use dotenvy::dotenv;
use std::env;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod plugins;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = init_db().await;

    let app = routes::init_routes(db.clone());

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: String = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
