mod issue;
mod search;
use std::{env, sync::Arc, time::Duration};

use axum::{
    Router,
    routing::{get, post},
};
use backend::archive::episode::{Episode, EpisodeInfo};
use dotenv_codegen::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    const DB_URL: &str = dotenv!("DATABASE_URL");

    // Connection pool for database connections
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(DB_URL)
        .await;

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
        .route("/search", get(search::search))
        .with_state(Arc::from(episodes))
        .route("/issue", post(issue::create_issue))
        .with_state(pool_arc.clone())
        .route("/issue", get(issue::get_issues))
        .with_state(pool_arc.clone());

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
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
