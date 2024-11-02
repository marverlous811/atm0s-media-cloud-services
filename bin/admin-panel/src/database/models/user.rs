use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

pub const USER_TABLE: &str = "d_users";

#[derive(Debug, Clone, WeldsModel, Serialize, Deserialize)]
#[welds(table = "d_users")]
#[welds(db(Sqlite, Postgres))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
pub struct User {
    #[welds(primary_key)]
    #[welds(column_type = "VARCHAR(32)")]
    pub id: String,
    #[welds(unique)]
    #[welds(column_type = "VARCHAR(255)")]
    pub email: String,
    #[welds(column_type = "VARCHAR(255)")]
    pub name: Option<String>,
    pub image: Option<String>,
    #[welds(sensitive)]
    #[welds(column_type = "VARCHAR(255)")]
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub async fn validate_user_table(client: &dyn welds::Client) -> anyhow::Result<Vec<welds::check::Issue>> {
    let issues = welds::check::schema::<User>(client).await?;
    Ok(issues
        .into_iter()
        .filter(|i| i.level == welds::check::Level::Critical)
        .collect())
}

fn before_create(user: &mut User) -> Result<()> {
    user.id = uuid::Uuid::new_v4().to_string();
    user.created_at = chrono::Utc::now().timestamp_millis();
    user.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn before_update(user: &mut User) -> Result<()> {
    user.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
