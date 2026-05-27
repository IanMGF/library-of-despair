mod search;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/search", get(search::search));

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
