use poem::{get, handler, web::Data, IntoResponse, Route};
use serde::Serialize;

use crate::http::HttpContext;

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub clerk_publishable_key: String,
}

#[handler]
pub async fn configs_view(data: Data<&HttpContext>) -> impl IntoResponse {
    http_common::response::to_response(Ok(ConfigResponse {
        clerk_publishable_key: data.cfg.clerk_publishable_key.clone(),
    }))
}

pub fn build_route() -> Route {
    Route::new().at("/view", get(configs_view))
}
