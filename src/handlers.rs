use crate::client::Clients;
use axum::extract::ws::WebSocketUpgrade;
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_connection(ws: WebSocketUpgrade, clients: Arc<Mutex<Clients>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| crate::client::handle_connection(socket, clients))
}
