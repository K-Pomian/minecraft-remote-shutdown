use async_channel::bounded;
use minecraft_remote_shutdown::user_api::entry::run;
use std::net::SocketAddr;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    println!("Initializing server...");

    let (api_kill_switch_sender, api_kill_switch_receiver) = bounded::<()>(1);
    let ip = SocketAddr::from(([127, 0, 0, 1], 3333));

    let _ = run(&ip, api_kill_switch_receiver).await.unwrap();
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");

    match api_kill_switch_sender.send(()).await {
        Ok(_) => {
            println!("Sent shutdown signal...");
        }
        Err(err) => {
            println!("Error sending shutdown signal: {}", err);
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("Got it! Exiting...");
}
