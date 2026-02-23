mod api;
mod db;
mod external;
mod ml;
mod models;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "premier_league_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = utils::config::Config::from_env()?;

    // Initialize database connection pool
    let db_pool = db::postgres::create_pool(&config.database_url).await?;

    // Initialize Redis connection
    let redis_client = redis::Client::open(config.redis_url.as_str())?;
    let redis_conn = redis_client.get_connection_manager().await?;

    // Load ML model
    let ml_model = ml::inference::load_model(&config.model_path)?;

    // Build application state
    let app_state = AppState {
        db_pool,
        redis_conn,
        ml_model,
        config,
    };

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api/v1", api::routes::create_routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Premier League Prediction API v1.0"
}

async fn health_check() -> &'static str {
    "OK"
}

// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub redis_conn: redis::aio::ConnectionManager,
    pub ml_model: std::sync::Arc<ml::inference::Model>,
    pub config: utils::config::Config,
}
