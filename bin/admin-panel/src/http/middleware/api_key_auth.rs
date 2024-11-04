use poem::{http::StatusCode, Endpoint, Error, IntoResponse, Middleware, Request, Response, Result};
use serde::Deserialize;

use crate::http::HttpContext;

const API_KEY_HEADER: &str = "api_key";

pub struct ApiKeyAuthMiddleware {
    ctx: HttpContext,
}

impl ApiKeyAuthMiddleware {
    pub fn new(ctx: HttpContext) -> Self {
        Self { ctx }
    }
}

impl<E: Endpoint> Middleware<E> for ApiKeyAuthMiddleware {
    type Output = ApiKeyAuthMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ApiKeyAuthMiddlewareImpl {
            endpoint: ep,
            ctx: self.ctx.clone(),
        }
    }
}

pub struct ApiKeyAuthMiddlewareImpl<E> {
    endpoint: E,
    ctx: HttpContext,
}

impl<E: Endpoint> Endpoint for ApiKeyAuthMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        fn get_api_key(req: &Request) -> anyhow::Result<String> {
            let api_key = req.headers().get(API_KEY_HEADER).and_then(|value| value.to_str().ok());
            if let Some(api_key) = api_key {
                return Ok(api_key.to_string());
            }

            #[derive(Clone, Deserialize)]
            struct Params {
                api_key: String,
            }

            let params = req.params::<Params>()?;
            Ok(params.api_key.clone())
        }

        match get_api_key(&req) {
            Ok(api_key) => {
                if api_key != self.ctx.cfg.api_key {
                    return Err(Error::from_string("Invalid api key", StatusCode::UNAUTHORIZED));
                }
                let res = self.endpoint.call(req).await;
                match res {
                    Ok(resp) => {
                        let resp = resp.into_response();
                        Ok(resp)
                    }
                    Err(err) => {
                        log::error!("[AuthMidlewareImpl] call endpoint error: {err}");
                        Err(err)
                    }
                }
            }
            Err(_) => return Err(Error::from_string("Missing api key", StatusCode::UNAUTHORIZED)),
        }
    }
}
