use axum::{routing::post, Router};
pub fn router() -> Router {
    Router::new()
        .route("/transition", post(|| async { "Transition state" }))
        .route("/shred", post(|| async { "Shred state" }))
}
