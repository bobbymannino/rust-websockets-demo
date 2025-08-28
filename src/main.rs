use axum::{routing::get, Router, response::IntoResponse};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	let app = Router::new().route("/health", get(health));

	let addr = SocketAddr::from(([0, 0, 0, 0], 2212));
	println!("Listening on http://{}", addr);

	 let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
	"ok"
}
