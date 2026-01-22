use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateDto {
    pub mount_status: String,

    pub gps_time: Option<String>,
    pub gps_fix: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    pub current_ra: Option<f64>,
    pub current_dec: Option<f64>,
    pub target_ra: Option<f64>,
    pub target_dec: Option<f64>,

    pub last_error: Option<String>,
}
