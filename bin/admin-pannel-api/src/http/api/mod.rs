use poem::{get, handler, IntoResponse, Route};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: bool,
    pub version: String,
}

#[handler]
pub async fn health_check() -> impl IntoResponse {
    http_common::response::to_response(Ok(HealthCheckResponse {
        status: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

pub fn build_route() -> Route {
    Route::new().nest("/health", get(health_check))
}
