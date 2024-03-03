use super::utils::build_router;
use anyhow::Result;
use async_channel::Receiver;
use axum::Router;
use std::net::SocketAddr;

pub async fn run(socket_address: &SocketAddr, api_kill_switch: Receiver<()>) -> Result<Router> {
    let router = build_router(socket_address, api_kill_switch).await;
    return Ok(router);
}
