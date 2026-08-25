use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/setup", post(|| async { "Setup DKG" }))
        .route("/nodes", get(|| async { "Get nodes" }))
}
