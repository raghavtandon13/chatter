mod client;
mod handlers;
mod types;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let clients = Arc::new(Mutex::new(client::Clients::new()));
    let app = Router::new()
        .route(
            "/chat",
            get(move |ws| handlers::handle_connection(ws, clients.clone())),
        )
        .route(
            "/hello",
            get(|| async { "Hello from Chatter messaging app." }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
