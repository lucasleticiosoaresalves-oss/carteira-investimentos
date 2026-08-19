mod auth;
mod config;
mod db;
mod extractors;
mod handlers;
mod models;
mod routes;
mod state;

use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let pool = db::create_pool(&config.database_url).await?;

    let state = AppState {
        pool,
        config: config.clone(),
    };

    let app = routes::build_router(state).layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Servidor rodando em http://{addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
