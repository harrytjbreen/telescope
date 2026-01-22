use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountStatus {
    Idle,
    Slewing,
    Tracking,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum GpsFix {
    NoFix,
    Fix2D,
    Fix3D,
}

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub ra: f64,
    pub dec: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Target {
    pub ra: f64,
    pub dec: f64,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub mount_status: MountStatus,

    pub gps_time: Option<DateTime<Utc>>,
    pub gps_fix: Option<GpsFix>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    pub current_pos: Option<Target>,
    pub target_pos: Option<Target>,

    pub last_error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mount_status: MountStatus::Idle,
            gps_time: None,
            gps_fix: None,
            latitude: None,
            longitude: None,
            current_pos: None,
            target_pos: None,
            last_error: None,
        }
    }
}
