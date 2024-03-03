use async_channel::bounded;
use log::{error, info};
use minecraft_remote_shutdown::user_api::entry::run;
use std::net::SocketAddr;
use std::sync::mpsc::channel;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    info!("Initializing server...");

    // Setup debug logs
    let filter: EnvFilter = "debug".parse().expect("filter should parse");
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Channels for graceful shutdown
    let (api_kill_switch_sender, api_kill_switch_receiver) = bounded::<()>(1);
    let ip = SocketAddr::from(([127, 0, 0, 1], 3333));

    // Start the server
    let _ = run(&ip, api_kill_switch_receiver).await.unwrap();
    let (tx, rx) = channel();

    // Setup server shutdown
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    info!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");

    match api_kill_switch_sender.send(()).await {
        Ok(_) => {
            info!("Sent shutdown signal...");
        }
        Err(err) => {
            error!("Error sending shutdown signal: {}", err);
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    info!("Got it! Exiting...");
}
