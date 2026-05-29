mod search;
use std::sync::Arc;

use axum::{Router, routing::get};
use backend::archive::episode::{Episode, EpisodeInfo};

#[tokio::main]
async fn main() {
    let episodes: Vec<Episode> = EpisodeInfo::get_episodes_list()
        .unwrap()
        .into_iter()
        .map(|info| info.load_episode().unwrap())
        .collect();

    let app = Router::new()
        .route("/search", get(search::search))
        .with_state(Arc::from(episodes));

    let listener_res = tokio::net::TcpListener::bind("0.0.0.0:3000").await;
    let listener = match listener_res {
        Ok(listener) => listener,
        Err(err) => panic!("Erro ao atrelar à porta: {err}\n"),
    };

    match axum::serve(listener, app).await {
        Ok(()) => {}
        Err(err) => panic!("Erro ao iniciar servidor:\n{err}\n"),
    }
}
