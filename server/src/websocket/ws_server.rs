use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::{
    app_state::{AppState, MountStatus, Target},
    websocket::ws_message::{ClientMessage, ServerMessage},
};

pub fn router(state: Arc<Mutex<AppState>>, tx: broadcast::Sender<ServerMessage>) -> Router {
    Router::new().route(
        "/ws",
        get(move |ws: WebSocketUpgrade| {
            let state = state.clone();
            let tx = tx.clone();

            async move { ws.on_upgrade(move |socket| handle_socket(socket, state, tx)) }
        }),
    )
}

async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<Mutex<AppState>>,
    tx: broadcast::Sender<ServerMessage>,
) {
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            // Client → Server
            Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(text) = msg {
                    if let Ok(cmd) = serde_json::from_str::<ClientMessage>(&text) {
                        match cmd {
                            ClientMessage::Goto { ra, dec } => {
                                let mut s = state.lock().unwrap();
                                s.target_pos = Some(Target { ra, dec });
                                s.mount_status = MountStatus::Slewing;
                            }

                            ClientMessage::StartTracking => {
                                state.lock().unwrap().mount_status =
                                    MountStatus::Tracking;
                            }

                            ClientMessage::StopTracking => {
                                state.lock().unwrap().mount_status =
                                    MountStatus::Idle;
                            }
                        }
                    }
                }
            }

            // Server → Client
            Ok(msg) = rx.recv() => {
                if let Ok(json) = serde_json::to_string(&msg) {
                    let _ = socket.send(Message::Text(json)).await;
                }
            }

            else => break,
        }
    }
}

