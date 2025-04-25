use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn init_db() -> AppState {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to connect to database: {:?}", e);
            panic!("Failed to connect to database: {}", e);
        }
    };

    // 测试数据库连接
    if let Err(e) = sqlx::query("SELECT 1").execute(&pool).await {
        tracing::error!("Database connection test failed: {:?}", e);
        panic!("Database connection test failed: {}", e);
    }

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    AppState { db: pool }
}
