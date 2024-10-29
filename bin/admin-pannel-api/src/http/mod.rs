mod api;
mod common;

use std::net::{Ipv4Addr, SocketAddr};

use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};

pub async fn run_http(port: u16) -> anyhow::Result<()> {
    let app = Route::new().nest("/api", api::build_route()).with(Tracing);

    let _ = Server::new(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)))
        .name("admin-pannel-api")
        .run(app)
        .await?;

    Ok(())
}
