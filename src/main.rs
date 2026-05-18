use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener_res = tokio::net::TcpListener::bind("0.0.0.0:3000").await;
    let listener = match listener_res {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Erro ao atrelar à porta: {e}\n");
            return;
        },
    };

    axum::serve(listener, app).await.unwrap();
}