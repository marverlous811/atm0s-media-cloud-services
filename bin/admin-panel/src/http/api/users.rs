use poem::{get, handler, web::Data, Error, IntoResponse, Request, Route};
use reqwest::StatusCode;

use crate::{
    database::{
        models::user::User,
        repositories::users::{get_user, UserFilterDto},
    },
    http::{HttpContext, USER_ID_HEADER},
};

#[handler]
pub async fn get_me(req: &Request, data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(req: &Request, data: Data<&HttpContext>) -> anyhow::Result<User> {
        let user_id = req.headers().get(USER_ID_HEADER).unwrap();
        let user_id = user_id.to_str().unwrap();
        let user = get_user(
            data.db.clone(),
            UserFilterDto {
                id: Some(user_id.to_string()),
                email: None,
            },
        )
        .await?;
        match user {
            Some(user) => Ok(user),
            None => anyhow::bail!(Error::from_string("User not found", StatusCode::NOT_FOUND)),
        }
    }

    http_common::response::to_response(process(req, data).await)
}

pub fn build_route() -> Route {
    Route::new().nest("/me", get(get_me))
}
