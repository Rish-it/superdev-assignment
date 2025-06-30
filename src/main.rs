mod errors;
mod handlers;
mod models;
mod utils;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use handlers::{create_token, generate_keypair, sign_message, verify_message};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/sign", post(sign_message))
        .route("/verify", post(verify_message))
        .layer(ServiceBuilder::new().layer(cors).into_inner());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Superdev Server running on http://{}", addr);
    println!("Endpoints:");
    println!("  GET  /health      - Health check");
    println!("  POST /keypair     - Generate new keypair");
    println!("  POST /token/create - Create SPL token mint instruction");
    println!("  POST /sign        - Sign a message");
    println!("  POST /verify      - Verify a signature");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "success": true,
        "data": {
            "status": "healthy",
            "service": "Superdev Server",
            "version": "0.1.0"
        }
    }))
}
