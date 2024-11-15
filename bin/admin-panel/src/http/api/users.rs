use clerk_rs::apis::users_api::User;
use poem::{get, handler, web::Data, IntoResponse, Route};

use crate::http::{middleware::clerk_auth::ClerkUserId, HttpContext};

#[handler]
pub async fn get_me(data: Data<&HttpContext>, user_id: ClerkUserId) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, user_id: String) -> anyhow::Result<clerk_rs::models::User> {
        match User::get_user(&data.clerk_client, user_id.as_str()).await {
            Ok(user) => Ok(user),
            Err(e) => anyhow::bail!(e),
        }
    }

    http_common::response::to_response(process(data, user_id.into()).await)
}

pub fn build_route() -> Route {
    Route::new().nest("/me", get(get_me))
}
