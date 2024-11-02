mod api;
mod middleware;
mod view;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use http_common::response::to_response_error;
use poem::{
    listener::TcpListener,
    middleware::{AddData, Tracing},
    EndpointExt, Route, Server,
};
use view::build_frontend_route;

const USER_ID_HEADER: &str = "X-User-Id";

#[derive(Clone)]
pub struct HttpCfg {
    pub jwt_secret: String,
    pub jwt_max_age_minutes: u64,
}

#[derive(Clone)]
pub struct HttpContext {
    pub db: Arc<dyn welds::Client>,
    pub cfg: HttpCfg,
}

pub async fn run_http(port: u16, db: Arc<dyn welds::Client>, cfg: HttpCfg) -> anyhow::Result<()> {
    let ctx = HttpContext { db, cfg };
    let app = Route::new()
        .nest("/api", api::build_route(ctx.clone()))
        .nest("/", build_frontend_route())
        .with(AddData::new(ctx))
        .with(Tracing)
        .catch_all_error(|e| async move { to_response_error(e.into()) });

    let _ = Server::new(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)))
        .name("admin-panel")
        .run(app)
        .await?;

    Ok(())
}
