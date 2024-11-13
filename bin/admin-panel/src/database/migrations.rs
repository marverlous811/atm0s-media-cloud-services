use welds::errors::Result;
use welds::migrations::{prelude::*, Manual};

use super::models::project::validate_project_table;

pub async fn check_tables(client: &dyn welds::Client) -> anyhow::Result<()> {
    let mut errors = vec![];
    errors.append(&mut validate_project_table(client).await?);

    if !errors.is_empty() {
        log::error!("Schema mismatch: found {:?} issues", errors);
        anyhow::bail!("Schema mismatch: found {} issues", errors.len());
    }

    Ok(())
}

pub async fn migration_up(client: &dyn welds::TransactStart) -> anyhow::Result<()> {
    match welds::migrations::up(client, &[m20241102_000000_init_admin_panel]).await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Failed to apply migrations: {:?}", e);
            anyhow::bail!("Failed to apply migrations: {:?}", e)
        }
    }
}

pub fn m20241102_000000_init_admin_panel(_: &TableState) -> Result<MigrationStep> {
    let up = include_str!("./migrations/20241102_0000000/up.sql");
    let down = include_str!("./migrations/20241102_0000000/down.sql");

    let m = Manual::up(up).down(down);

    Ok(MigrationStep::new("init_admin_panel", m))
}
