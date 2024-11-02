use super::models::user::{validate_user_table, USER_TABLE};
use welds::errors::Result;
use welds::migrations::prelude::*;

pub async fn check_tables(client: &dyn welds::Client) -> anyhow::Result<()> {
    let mut errors = vec![];
    errors.append(&mut validate_user_table(client).await?);

    if !errors.is_empty() {
        log::error!("Schema mismatch: found {:?} issues", errors);
        anyhow::bail!("Schema mismatch: found {} issues", errors.len());
    }

    Ok(())
}

pub async fn migration_up(client: &dyn welds::TransactStart) -> anyhow::Result<()> {
    match welds::migrations::up(client, &[m20241102_000000_create_user_table]).await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Failed to apply migrations: {:?}", e);
            anyhow::bail!("Failed to apply migrations: {:?}", e)
        }
    }
}

pub fn m20241102_000000_create_user_table(_: &TableState) -> Result<MigrationStep> {
    let m = create_table(USER_TABLE)
        .id(|c| c("id", Type::StringSized(32)))
        .column(|c| c("email", Type::StringSized(255)).create_unique_index())
        .column(|c| c("name", Type::StringSized(255)).is_null())
        .column(|c| c("image", Type::String).is_null())
        .column(|c| c("password", Type::StringSized(255)).is_null())
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));

    Ok(MigrationStep::new("create_user_table", m))
}
