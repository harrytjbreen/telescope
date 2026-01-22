use serde::{Deserialize, Serialize};

use crate::websocket::app_state_dto::AppStateDto;

// client → server
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Goto { ra: f64, dec: f64 },
    StartTracking,
    StopTracking,
}

// server → client
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    State { data: AppStateDto },
    Error { message: String },
}
