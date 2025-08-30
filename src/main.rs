use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
    http::{HeaderMap, StatusCode},
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

async fn before_ws(headers: HeaderMap, ws: WebSocketUpgrade) -> impl IntoResponse {
    // Hardcoded bearer token for authentication
    const VALID_TOKEN: &str = "secret-websocket-token";
    
    // Extract the Authorization header
    let auth_header = headers.get("authorization");
    
    // Check if the Authorization header is present and valid
    match auth_header {
        Some(header_value) => {
            let auth_str = header_value.to_str().unwrap_or("");
            
            // Check if it starts with "Bearer " and extract the token
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if token == VALID_TOKEN {
                    // Valid token, proceed with WebSocket upgrade
                    ws.on_upgrade(ws_handler)
                } else {
                    // Invalid token
                    (StatusCode::UNAUTHORIZED, "Invalid bearer token").into_response()
                }
            } else {
                // Authorization header doesn't start with "Bearer "
                (StatusCode::UNAUTHORIZED, "Invalid authorization header format").into_response()
            }
        }
        None => {
            // No Authorization header present
            (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response()
        }
    }
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
