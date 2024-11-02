mod api;
mod view;

use std::net::{Ipv4Addr, SocketAddr};

use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use view::build_frontend_route;

pub async fn run_http(port: u16) -> anyhow::Result<()> {
    let app = Route::new()
        .nest("/api", api::build_route())
        .nest("/", build_frontend_route())
        .with(Tracing);

    let _ = Server::new(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)))
        .name("admin-pannel-api")
        .run(app)
        .await?;

    Ok(())
}
