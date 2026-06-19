mod search;
use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderValue, header::CONTENT_TYPE},
    routing::get,
};
use common_types::archive::episode::{Episode, EpisodeInfo};
use dotenv_codegen::dotenv;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    const CORS_ORIGIN: &str = dotenv!("CORS_ORIGIN");
    let port = std::env::var("PORT").expect("environment variable PORT must be set");

    // Load episode information into main memory
    let episodes: Vec<Episode> = EpisodeInfo::get_episodes_list()
        .unwrap()
        .into_iter()
        .map(|info| info.load_episode().unwrap())
        .collect();

    let allowed_cors_origins = [CORS_ORIGIN.parse::<HeaderValue>().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(allowed_cors_origins)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/line", get(search::search))
        .with_state(Arc::from(episodes))
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
