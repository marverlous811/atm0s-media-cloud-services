use poem::{
    handler, post,
    web::{Data, Json},
    IntoResponse, Route,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        models::user::User,
        repositories::users::{create_user, get_user, CreateUserDto, UserFilterDto},
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

    http_common::response::to_response(process(body, data).await)
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
}
