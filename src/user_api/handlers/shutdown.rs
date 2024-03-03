use axum::http::StatusCode;
use axum::Json;

pub async fn shutdown_machine() -> Result<Json<()>, (StatusCode, String)> {
    match system_shutdown::shutdown() {
        Ok(_) => Ok(Json(())),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}
