mod google;

use http_common::response::to_response_error;
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use poem::{
    get, handler,
    http::StatusCode,
    post,
    web::{Data, Json, Query},
    Error, IntoResponse, Response, Route,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        models::user::User,
        repositories::{
            account::{create_account, get_account, AccountFilterDto, CreateAccountDto},
            users::{create_user, get_user, CreateUserDto, UserFilterDto},
        },
    },
    http::HttpContext,
};

#[derive(Debug, Deserialize, Clone)]
pub struct UserRegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTokenInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenClaims {
    pub info: UserTokenInfo,
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserSessionResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
}

#[handler]
pub async fn register(body: Json<UserRegisterRequest>, data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(body: Json<UserRegisterRequest>, data: Data<&HttpContext>) -> anyhow::Result<User> {
        let hashed_password = bcrypt::hash(body.password.clone(), bcrypt::DEFAULT_COST)?;

        let user = create_user(
            data.db.clone(),
            CreateUserDto {
                email: body.email.clone(),
                password: Some(hashed_password),
                name: None,
                image: None,
            },
        )
        .await?;

        Ok(user)
    }

    http_common::response::to_response(process(body, data).await)
}

#[handler]
pub async fn login(body: Json<UserRegisterRequest>, data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(body: Json<UserRegisterRequest>, data: Data<&HttpContext>) -> anyhow::Result<UserSessionResponse> {
        let user = get_user(
            data.db.clone(),
            UserFilterDto {
                email: Some(body.email.clone()),
                id: None,
            },
        )
        .await?;

        match user {
            Some(user) => {
                if let Some(password) = user.password.clone() {
                    let compare_password = bcrypt::verify(body.password.clone(), &password)?;
                    if !compare_password {
                        anyhow::bail!("Invalid password");
                    }
                } else {
                    anyhow::bail!("User not have password");
                }

                let token = generate_jwt_token(user, data.cfg.jwt_secret.clone(), data.cfg.jwt_max_age_minutes)?;

                Ok(UserSessionResponse { access_token: token })
            }
            None => anyhow::bail!(Error::from_string("User not found", StatusCode::NOT_FOUND)),
        }
    }

    http_common::response::to_response(process(body, data).await)
}

#[handler]
pub fn google_oauth_login(data: Data<&HttpContext>) -> impl IntoResponse {
    let redirect_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={}&response_type=code&redirect_uri={}",
        data.cfg.google_id, data.cfg.google_redirect_uri
    );

    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", redirect_url)
        .body("")
}

#[handler]
pub async fn google_oauth_handler(query: Query<QueryCode>, data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(query: Query<QueryCode>, data: Data<&HttpContext>) -> anyhow::Result<UserSessionResponse> {
        let code = &query.code;

        if code.is_empty() {
            anyhow::bail!(Error::from_string("Code is empty", StatusCode::BAD_REQUEST));
        }

        // log::info!("will exchange code: {}", code);
        let token_response = data
            .google_oauth_client
            .clone()
            .exchange_code(AuthorizationCode::new(code.clone()))
            .request_async(async_http_client)
            .await?;

        // log::info!("token response: {:?}", token_response);

        let profile = data
            .http_client
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(token_response.access_token().secret().to_owned())
            .send()
            .await?;

        let google_user = profile.json::<google::GoogleUserResult>().await?;

        let user = get_user(
            data.db.clone(),
            UserFilterDto {
                email: Some(google_user.email.clone()),
                id: None,
            },
        )
        .await?;

        let user = match user {
            Some(user) => user,
            None => {
                create_user(
                    data.db.clone(),
                    CreateUserDto {
                        email: google_user.email.clone(),
                        password: None,
                        name: Some(google_user.name.clone()),
                        image: Some(google_user.picture.clone()),
                    },
                )
                .await?
            }
        };

        let account = get_account(
            data.db.clone(),
            AccountFilterDto {
                user_id: Some(user.id.clone()),
                provider: Some("google".to_string()),
            },
        )
        .await?;

        let _ = match account {
            Some(account) => account,
            None => {
                create_account(
                    data.db.clone(),
                    CreateAccountDto {
                        user_id: user.id.clone(),
                        provider: "google".to_string(),
                        provider_account_id: google_user.sub.clone(),
                        access_token: Some(token_response.access_token().secret().clone()),
                        refresh_token: token_response.refresh_token().map(|r| r.secret().clone()),
                        expires_at: token_response.expires_in().map(|r| r.as_secs() as i64),
                        token_type: Some(token_response.token_type().as_ref().to_string()),
                        scope: token_response
                            .scopes()
                            .map(|r| r.into_iter().map(|s| s.to_string()).collect()),
                        auth_type: "oauth".to_string(),
                    },
                )
                .await?
            }
        };

        let token = generate_jwt_token(user, data.cfg.jwt_secret.clone(), data.cfg.jwt_max_age_minutes)?;

        Ok(UserSessionResponse { access_token: token })
    }

    let cookie_secure = if data.cfg.is_secure {
        "Secure"
    } else {
        ""
    };

    match process(query, data).await {
        Ok(res) => Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(
                "Set-Cookie",
                format!("access_token={}; Path=/; HttpOnly; {cookie_secure}", res.access_token),
            )
            .header("Location", "/")
            .body(""),
        Err(e) => to_response_error(e),
    }
}

fn generate_jwt_token(user: User, secret: String, max_age_minutes: u64) -> anyhow::Result<String> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(max_age_minutes as i64)).timestamp() as usize;

    let claims = UserTokenClaims {
        iat,
        exp,
        sub: user.id.to_string(),
        info: UserTokenInfo {
            id: user.id.to_string(),
            email: user.email.clone(),
            name: user.name.clone(),
            image: user.image.clone(),
        },
    };

    Ok(jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn build_route() -> Route {
    Route::new()
        .nest("/register", post(register))
        .nest("/login", post(login))
        .nest("/google", get(google_oauth_login))
        .nest("/callback/google", get(google_oauth_handler))
}
