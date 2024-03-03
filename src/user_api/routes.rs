use super::{handlers::get_time::get_time, utils::get_cors};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn routes() -> Router {
    // Setup CORS and routes
    let cors = get_cors();
    let mut router = Router::new()
        .route("/get_time", get(get_time))
        .layer(TraceLayer::new_for_http());

    router = router.layer(cors);
    router
}
