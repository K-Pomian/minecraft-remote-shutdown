use crate::user_api::utils::get_timestamp_in_seconds;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeResponse {
    pub time: String,
}

pub async fn get_time() -> Result<Json<TimeResponse>, (StatusCode, String)> {
    let resp = TimeResponse {
        time: get_timestamp_in_seconds().to_string(),
    };
    return Ok(Json(resp));
}
