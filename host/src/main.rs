use axum::{
    routing::{get, post, delete, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

mod db;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize DB
    db::init_db().expect("Failed to initialize database");

    let app = Router::new()
        .nest("/v1/secrets", routes::secrets::router())
        .nest("/v1/keys", routes::keys::router())
        .nest("/v1/tokens", routes::tokens::router())
        .nest("/v1/attest", routes::attest::router())
        .nest("/v1/lifecycle", routes::lifecycle::router())
        .nest("/v1/dkg", routes::dkg::router())
        .nest("/v1/entropy", routes::entropy::router())
        .route("/health", get(|| async { "OK" }))
        .nest_service("/", ServeDir::new("gui/dist"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
