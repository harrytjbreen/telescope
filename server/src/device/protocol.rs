use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "track")]
    StartTracking,

    #[serde(rename = "stop")]
    StopTracking,

    #[serde(rename = "goto")]
    GoTo { ra: f64, dec: f64 },

    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "ok")]
    Ok,

    #[serde(rename = "pong")]
    Pong,

    #[serde(rename = "error")]
    Error { message: String },

    #[serde(rename = "status")]
    Status { tracking: bool },

    #[serde(rename = "gps")]
    Gps { lat: f64, lon: f64, fix: u8 },

    #[serde(rename = "slew")]
    Slew {
        current_ra: f64,
        current_dec: f64,
        target_ra: f64,
        target_dec: f64,
    },
}
