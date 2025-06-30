mod errors;
mod handlers;
mod models;
mod utils;

use axum::{
    http::Method,
    routing::post,
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
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/sign", post(sign_message))
        .route("/verify", post(verify_message))
        .layer(ServiceBuilder::new().layer(cors).into_inner());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Solana HTTP Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
