use super::routes::routes;
use async_channel::Receiver;
use axum::{http::Method, Router};
use futures::StreamExt;
use std::{
    net::SocketAddr,
    pin::Pin,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tower_http::cors::{Any, CorsLayer};

pub fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ])
        .max_age(Duration::from_secs(86400))
        .vary(vec![
            axum::http::header::ORIGIN,
            axum::http::header::ACCESS_CONTROL_REQUEST_METHOD,
            axum::http::header::ACCESS_CONTROL_REQUEST_HEADERS,
        ])
}

pub fn get_timestamp_in_seconds() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub async fn build_router(socket_addr: &SocketAddr, api_kill_switch: Receiver<()>) -> Router {
    let router = routes().await;

    let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();
    let pinned_api_kill_switch = Box::pin(api_kill_switch);

    let graceful_shutdown = axum::serve(
        listener,
        router
            .clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(pinned_api_kill_switch));

    tokio::spawn(async move {
        match graceful_shutdown.await {
            Ok(_) => println!("Server shutdown gracefully"),
            Err(e) => println!("Server shutdown with error: {}", e),
        }
    });
    return router;
}

async fn shutdown_signal(mut receiver: Pin<Box<Receiver<()>>>) {
    match receiver.next().await {
        Some(_) => {
            println!("Shutting down Axum server...");
        }
        None => {
            println!(
            "Error has occurred, maybe the channel is closed ? Shutting down Axum server anyway..."
            );
        }
    }
}
