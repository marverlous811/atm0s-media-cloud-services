use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::database::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub name: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilterDto {
    pub email: Option<String>,
    pub id: Option<String>,
}

pub async fn create_user(client: Arc<dyn welds::Client>, dto: CreateUserDto) -> anyhow::Result<User> {
    let mut user = User::new();
    user.email = dto.email.clone();
    user.name = dto.name.clone();
    user.image = dto.image.clone();
    user.password = dto.password.clone();

    user.save(client.as_ref()).await?;
    Ok(user.into_inner())
}

pub async fn get_user(client: Arc<dyn welds::Client>, filter: UserFilterDto) -> anyhow::Result<User> {
    let query = build_filter(filter).limit(1);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(user) => Ok(user.into_inner()),
        None => anyhow::bail!("User not found"),
    }
}

fn build_filter(filter: UserFilterDto) -> welds::query::builder::QueryBuilder<User> {
    let mut query = User::all();
    if let Some(email) = filter.email {
        query = query.where_col(|c| c.email.equal(email.clone()));
    }
    if let Some(id) = filter.id {
        query = query.where_col(|c| c.id.equal(id.clone()));
    }

    query
}
