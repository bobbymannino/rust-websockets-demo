use axum::{
    Router,
    extract::{
        ConnectInfo, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get(before_ws));

    let addr = SocketAddr::from(([0, 0, 0, 0], 2212));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
    "ok"
}

async fn before_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(ws_handler)
}

async fn ws_handler(mut socket: WebSocket) {
    println!("WebSocket connected");

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                println!("Received text: {}", text);

                match socket.send(Message::Text(text)).await {
                    Ok(()) => {
                        println!("Message mirrored successfully");
                    }
                    Err(err) => {
                        eprintln!("Failed to send message: {}", err);
                    }
                }
            }
            _ => {}
        }
    }
}
