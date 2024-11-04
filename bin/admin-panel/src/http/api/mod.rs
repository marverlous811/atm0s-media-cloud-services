mod auth;
mod projects;
mod users;

pub use auth::UserTokenClaims;

use poem::{get, handler, EndpointExt, IntoResponse, Route};
use serde::Serialize;

use super::{middleware, HttpContext};

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

pub fn build_route(ctx: HttpContext) -> Route {
    Route::new()
        .nest("/health", get(health_check))
        .nest("/auth", auth::build_route())
        .nest(
            "/users",
            users::build_route().with(middleware::auth::AuthMiddleware::new(ctx.clone())),
        )
        .nest("/projects", projects::build_route(ctx.clone()))
}
