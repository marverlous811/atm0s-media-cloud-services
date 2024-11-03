use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::database::models::account::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountDto {
    pub user_id: String,
    pub auth_type: String,
    pub provider: String,
    pub provider_account_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountFilterDto {
    pub user_id: Option<String>,
    pub provider: Option<String>,
}

pub async fn create_account(client: Arc<dyn welds::Client>, dto: CreateAccountDto) -> anyhow::Result<Account> {
    let mut account = Account::new();
    account.user_id = dto.user_id.clone();
    account.auth_type = dto.auth_type.clone();
    account.provider = dto.provider.clone();
    account.provider_account_id = dto.provider_account_id.clone();
    account.access_token = dto.access_token.clone();
    account.refresh_token = dto.refresh_token.clone();
    account.expires_at = dto.expires_at.clone();
    account.token_type = dto.token_type.clone();
    account.scope = dto.scope.clone();

    account.save(client.as_ref()).await?;
    Ok(account.into_inner())
}

pub async fn get_account(client: Arc<dyn welds::Client>, filter: AccountFilterDto) -> anyhow::Result<Option<Account>> {
    let query = build_filter(filter).limit(1);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(account) => Ok(Some(account.into_inner())),
        None => Ok(None),
    }
}

fn build_filter(filter: AccountFilterDto) -> welds::query::builder::QueryBuilder<Account> {
    let mut query = Account::all();
    if let Some(user_id) = filter.user_id {
        query = query.where_col(|c| c.user_id.equal(user_id.clone()));
    }
    if let Some(provider) = filter.provider {
        query = query.where_col(|c| c.provider.equal(provider.clone()));
    }
    query
}
