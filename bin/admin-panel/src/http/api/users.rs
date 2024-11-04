use poem::{get, handler, web::Data, Error, IntoResponse, Route};
use reqwest::StatusCode;

use crate::{
    database::{
        models::user::User,
        repositories::users::{get_user, UserFilterDto},
    },
    http::{middleware::auth::UserId, HttpContext},
};

#[handler]
pub async fn get_me(data: Data<&HttpContext>, user_id: UserId) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, user_id: UserId) -> anyhow::Result<User> {
        let user = get_user(
            data.db.clone(),
            UserFilterDto {
                id: Some(user_id.into()),
                email: None,
            },
        )
        .await?;
        match user {
            Some(user) => Ok(user),
            None => anyhow::bail!(Error::from_string("User not found", StatusCode::NOT_FOUND)),
        }
    }

    http_common::response::to_response(process(data, user_id).await)
}

pub fn build_route() -> Route {
    Route::new().nest("/me", get(get_me))
}
