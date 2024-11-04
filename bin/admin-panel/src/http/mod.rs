mod api;
mod middleware;
mod view;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use http_common::response::to_response_error;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl};
use poem::{
    listener::TcpListener,
    middleware::{AddData, Tracing},
    session::{CookieConfig, CookieSession},
    EndpointExt, Route, Server,
};
use view::build_frontend_route;

#[derive(Clone)]
pub struct HttpCfg {
    pub jwt_secret: String,
    pub jwt_max_age_minutes: u64,
    pub is_secure: bool,
    pub google_id: String,
    pub google_secret: String,
    pub google_redirect_uri: String,
    pub api_key: String,
}

#[derive(Clone)]
pub struct HttpContext {
    pub db: Arc<dyn welds::Client>,
    pub http_client: reqwest::Client,
    google_oauth_client: BasicClient,
    pub cfg: HttpCfg,
}

pub async fn run_http(port: u16, db: Arc<dyn welds::Client>, cfg: HttpCfg) -> anyhow::Result<()> {
    let google_client_id = ClientId::new(cfg.google_id.clone());
    let google_client_secret = ClientSecret::new(cfg.google_secret.clone());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url =
        TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).expect("Invalid token endpoint URL");
    let google_oauth_client = BasicClient::new(google_client_id, Some(google_client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new(cfg.google_redirect_uri.clone()).unwrap())
        .set_revocation_uri(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                .expect("Invalid revocation endpoint URL"),
        );

    let ctx = HttpContext {
        db,
        http_client: reqwest::Client::new(),
        google_oauth_client,
        cfg: cfg.clone(),
    };

    let app = Route::new()
        .nest("/api", api::build_route(ctx.clone()))
        .nest("/", build_frontend_route())
        .with(AddData::new(ctx))
        .with(CookieSession::new(CookieConfig::default().secure(cfg.is_secure)))
        .with(Tracing)
        .catch_all_error(|e| async move { to_response_error(e.into()) });

    let _ = Server::new(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)))
        .name("admin-panel")
        .run(app)
        .await?;

    Ok(())
}
