use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "List secrets" }))
        .route("/", post(|| async { "Create secret" }))
}
