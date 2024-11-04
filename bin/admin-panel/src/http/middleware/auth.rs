use poem::{
    http::{HeaderValue, StatusCode},
    Endpoint, Error, FromRequest, IntoResponse, Middleware, Request, RequestBody, Response, Result,
};

use crate::{
    database::repositories::users::{get_user, UserFilterDto},
    http::{api::UserTokenClaims, HttpContext},
};

const USER_ID_HEADER: &str = "X-User-Id";

pub struct UserId(String);

impl UserId {
    pub fn into(self) -> String {
        self.0
    }
}

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let user_id = req
            .headers()
            .get(USER_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("missing user_id", StatusCode::BAD_REQUEST))?;
        Ok(UserId(user_id.to_string()))
    }
}

pub struct AuthMiddleware {
    ctx: HttpContext,
}

impl AuthMiddleware {
    pub fn new(ctx: HttpContext) -> Self {
        Self { ctx }
    }
}

impl<E: Endpoint> Middleware<E> for AuthMiddleware {
    type Output = AuthMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthMiddlewareImpl {
            endpoint: ep,
            ctx: self.ctx.clone(),
        }
    }
}

pub struct AuthMiddlewareImpl<E> {
    endpoint: E,
    ctx: HttpContext,
}

impl<E: Endpoint> Endpoint for AuthMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let mut req = req;
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Error::from_string("Missing authorization header", StatusCode::UNAUTHORIZED))?;

        token
            .starts_with("Bearer ")
            .then_some(())
            .ok_or(Error::from_string("Not Bearer token", StatusCode::BAD_REQUEST))?;
        let token = &token[7..];
        log::info!("[AuthMidlewareImpl] got token {token}");

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

        match jsonwebtoken::decode::<UserTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.ctx.cfg.jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(claims) => {
                log::info!("[AuthMidlewareImpl] decode token success: {claims:?}");
                let user = get_user(
                    self.ctx.db.clone(),
                    UserFilterDto {
                        id: Some(claims.claims.sub.clone()),
                        email: None,
                    },
                )
                .await;

                if let Err(e) = user {
                    log::error!("[AuthMidlewareImpl] get user error: {e}");
                    return Err(Error::from_string("User not found", StatusCode::UNAUTHORIZED));
                }

                let user = user.unwrap().unwrap();
                let headers = req.headers_mut();
                headers.insert(USER_ID_HEADER, HeaderValue::from_str(&user.id).unwrap());

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
            Err(e) => {
                log::error!("[AuthMidlewareImpl] decode token error: {e}");
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        log::error!("[AuthMidlewareImpl] invalid token");
                        return Err(Error::from_string("Invalid token", StatusCode::UNAUTHORIZED));
                    }
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        log::error!("[AuthMidlewareImpl] token expired");
                        return Err(Error::from_string("Token expired", StatusCode::UNAUTHORIZED));
                    }
                    _ => {
                        log::error!("[AuthMidlewareImpl] unknown error: {e}");
                        return Err(Error::from_string("Invalid token", StatusCode::UNAUTHORIZED));
                    }
                }
            }
        }
    }
}
