use super::{
    handlers::{get_time::get_time, shutdown::shutdown_machine},
    utils::get_cors,
};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn routes() -> Router {
    // Setup CORS and routes
    let cors = get_cors();
    let mut router = Router::new()
        .route("/get_time", get(get_time))
        .route("/shutdown", get(shutdown_machine))
        .layer(TraceLayer::new_for_http());

    router = router.layer(cors);
    router
}
