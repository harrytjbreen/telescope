mod app_state;
mod db;
mod device;
mod websocket;

use dotenv::dotenv;
use std::sync::{Arc, Mutex};

use tokio::{net::TcpListener, sync::broadcast};

use crate::{
    app_state::AppState,
    device::serial::spawn_serial_link,
    device::protocol::Command,
    websocket::{ws_message::ServerMessage, ws_server},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _db = db::init_db().await.expect("DB init failed");

    let state = Arc::new(Mutex::new(AppState::new()));

    let (tx, _) = broadcast::channel::<ServerMessage>(32);

    let port = std::env::var("SERIAL_PORT").unwrap_or_else(|_| "/dev/cu.ttys007".to_string());

    eprintln!("{}", port);

    let cmd_tx = spawn_serial_link(port, 115200, state.clone());

    // Periodically send a command over serial
    tokio::spawn(async move {
        use tokio::time::{sleep, Duration};

        loop {
            if let Err(e) = cmd_tx.send(Command::Ping).await {
                eprintln!("Failed to send command over serial: {e}");
                break;
            }

            eprintln!("Sending ping command");
            sleep(Duration::from_secs(2)).await;
        }
    });

    let app = ws_server::router(state.clone(), tx);

    println!("WebSocket listening on ws://localhost:3000/ws");

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to bind to port 3000");

    axum::serve(listener, app).await.expect("Server crashed");
}
