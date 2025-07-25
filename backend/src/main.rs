use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::fmt::init;

mod db;
mod handlers;

use db::Database;
use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/work_tracker".to_string());

    info!("Connecting to database: {}", database_url);
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let db = Arc::new(Database::new(pool));
    let app_state = AppState { db };

    // Configure CORS for Railway deployment
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any) // In production, consider restricting to specific domains
        .allow_credentials(false);

    let app = Router::new()
        .route("/api/sessions", get(get_sessions))
        .route("/api/sessions", post(create_session))
        .route("/api/sessions/:id", get(get_session))
        .route("/api/sessions/:id", put(update_session))
        .route("/api/sessions/:id", delete(delete_session))
        .route("/api/tags", get(get_tags))
        .route("/api/tags", post(create_tag))
        .route("/api/tags/:id", get(get_tag))
        .route("/api/tags/:id", put(update_tag))
        .route("/api/tags/:id", delete(delete_tag))
        .layer(cors)
        .with_state(app_state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("Server running on http://{}", bind_address);
    
    axum::serve(listener, app).await?;

    Ok(())
}