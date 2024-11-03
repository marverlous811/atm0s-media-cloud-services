use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

use super::user::User;

pub const ACCOUNT_TABLE: &str = "d_accounts";

#[derive(Debug, Clone, WeldsModel, Serialize, Deserialize)]
#[welds(table = "d_accounts")]
#[welds(db(Sqlite, Postgres))]
#[welds(BelongsTo(user, User, "user_id"))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
pub struct Account {
    #[welds(primary_key)]
    pub id: String,

    pub user_id: String,

    #[welds(rename = "type")]
    #[welds(column_type = "VARCHAR(255)")]
    pub auth_type: String,

    #[welds(column_type = "VARCHAR(255)")]
    pub provider: String,
    #[welds(column_type = "VARCHAR(255)")]
    pub provider_account_id: String,

    #[welds(column_type = "VARCHAR(255)")]
    pub access_token: Option<String>,
    #[welds(column_type = "VARCHAR(255)")]
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    #[welds(column_type = "VARCHAR(255)")]
    pub token_type: Option<String>,
    #[welds(column_type = "VARCHAR(255)")]
    pub scope: Option<String>,
    #[welds(column_type = "VARCHAR(255)")]
    pub session_state: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}

fn before_create(account: &mut Account) -> Result<()> {
    account.id = uuid::Uuid::new_v4().to_string();
    account.created_at = chrono::Utc::now().timestamp_millis();
    account.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn before_update(account: &mut Account) -> Result<()> {
    account.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
