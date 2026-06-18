mod issue;
mod search;
use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    http::{HeaderValue, header::CONTENT_TYPE},
    routing::{get, post},
};
use backend::archive::episode::{Episode, EpisodeInfo};
use dotenv_codegen::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    const DB_URL: &str = dotenv!("DATABASE_URL");
    const CORS_ORIGIN: &str = dotenv!("CORS_ORIGIN");
    let port = std::env::var("PORT").expect("environment variable PORT must be set");

    // Connection pool for database connections
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(DB_URL)
        .await;

    let allowed_cors_origins = [CORS_ORIGIN.parse::<HeaderValue>().unwrap()];
    let cors = CorsLayer::new().allow_origin(allowed_cors_origins).allow_headers([CONTENT_TYPE]);

    let pool = match pool {
        Ok(p) => p,
        Err(err) => panic!("Erro abrindo pool de conexões com o banco: {err}"),
    };

    let pool_arc = Arc::new(pool);

    // Load episode information into main memory
    let episodes: Vec<Episode> = EpisodeInfo::get_episodes_list()
        .unwrap()
        .into_iter()
        .map(|info| info.load_episode().unwrap())
        .collect();

    let app = Router::new()
        .route("/line", get(search::search))
        .with_state(Arc::from(episodes))
        .route("/issue", post(issue::create_issue))
        .with_state(pool_arc.clone())
        .route("/issue", get(issue::get_issues))
        .with_state(pool_arc.clone())
        .layer(cors);

    let listener_res = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await;
    let listener = match listener_res {
        Ok(listener) => listener,
        Err(err) => panic!("Erro ao atrelar à porta: {err}\n"),
    };

    match axum::serve(listener, app).await {
        Ok(()) => {}
        Err(err) => panic!("Erro ao iniciar servidor:\n{err}\n"),
    }
}
