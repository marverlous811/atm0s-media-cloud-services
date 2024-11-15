use welds::errors::Result;
use welds::migrations::prelude::*;

pub fn up<'a>() -> Vec<MigrationFn> {
    vec![
        m20241102_create_projects,
        m20241102_create_project_invites,
        m20241102_create_project_members,
    ]
}

pub fn m20241102_create_projects(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("d_projects")
        .id(|c| c("id", Type::String))
        .column(|c| c("owner", Type::String))
        .column(|c| c("name", Type::String))
        .column(|c| c("secret", Type::String).create_unique_index())
        .column(|c| c("options", Type::String))
        .column(|c| c("codecs", Type::String))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));
    Ok(MigrationStep::new("create projects", m))
}

pub fn m20241102_create_project_invites(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("t_project_invites")
        .id(|c| c("id", Type::String))
        .column(|c| c("project_id", Type::String).create_index())
        .column(|c| c("email", Type::String))
        .column(|c| c("role", Type::String))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("expire_at", Type::IntBig));

    // .foreign_key("project_id", "d_projects", "id", Some("CASCADE"));
    Ok(MigrationStep::new("create project invites", m))
}

pub fn m20241102_create_project_members(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("d_project_members")
        .id(|c| c("id", Type::Int))
        .column(|c| c("project_id", Type::String).create_index())
        .column(|c| c("user_id", Type::String).create_index())
        .column(|c| c("role", Type::String))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));
    // .foreign_key("project_id", "d_projects", "id", Some("CASCADE"));
    Ok(MigrationStep::new("create project members", m))
}
