#![allow(dead_code)]

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct Client {
    pub id: Uuid,
    pub sender: Arc<Mutex<futures::stream::SplitSink<WebSocket, Message>>>,
}

pub type Clients = HashMap<Uuid, Client>;

impl Client {
    pub fn new(id: Uuid, sender: futures::stream::SplitSink<WebSocket, Message>) -> Self {
        Client {
            id,
            sender: Arc::new(Mutex::new(sender)),
        }
    }
}

pub async fn handle_connection(ws: WebSocket, clients: Arc<Mutex<Clients>>) {
    let (sender, mut receiver) = ws.split();
    let id = Uuid::new_v4();
    {
        let mut clients = clients.lock().await;
        clients.insert(id, Client::new(id, sender));
    }
    println!("Client connected: {}", id);

    while let Some(result) = receiver.next().await {
        match result {
            Ok(msg) => handle_message(id, msg, &clients).await,
            Err(e) => {
                eprintln!("Error receiving message from client {}: {}", id, e);
                break;
            }
        }
    }

    let mut clients = clients.lock().await;
    clients.remove(&id);
    println!("Client disconnected: {}", id);
}

async fn handle_message(id: Uuid, msg: Message, clients: &Arc<Mutex<Clients>>) {
    let msg_text = if let Message::Text(text) = msg {
        text
    } else {
        return;
    };

    let clients = clients.lock().await;
    for (client_id, client) in clients.iter() {
        if client_id != &id {
            if let Err(e) = client
                .sender
                .lock()
                .await
                .send(Message::Text(msg_text.clone()))
                .await
            {
                eprintln!("Error sending message to client {}: {}", client_id, e);
            }
        }
    }
}
