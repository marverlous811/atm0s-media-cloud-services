use std::sync::Arc;

use crate::database::models::project_member::ProjectInvite;

#[derive(Debug, Clone)]
pub struct CreateProjectInviteDto {
    pub email: String,
    pub project_id: String,
    pub expire_at: i64,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct ProjectInviteFilterDto {
    pub id: Option<String>,
    pub email: Option<String>,
    pub project_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectInviteUpdateDto {
    pub expire_at: i64,
}

pub async fn create_project_invite(
    client: Arc<dyn welds::Client>,
    dto: CreateProjectInviteDto,
) -> anyhow::Result<ProjectInvite> {
    let mut invite = ProjectInvite::new();
    invite.expire_at = dto.expire_at;
    invite.email = dto.email;
    invite.project_id = dto.project_id;
    invite.role = dto.role;

    invite.save(client.as_ref()).await?;
    Ok(invite.into_inner())
}

pub async fn get_project_invite(
    client: Arc<dyn welds::Client>,
    dto: ProjectInviteFilterDto,
) -> anyhow::Result<Option<ProjectInvite>> {
    let query = build_project_invite_query(dto);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(invite) => Ok(Some(invite.into_inner())),
        None => Ok(None),
    }
}

pub async fn delete_project_invite(client: Arc<dyn welds::Client>, id: String) -> anyhow::Result<bool> {
    let invite = ProjectInvite::find_by_id(client.as_ref(), id).await?;

    match invite {
        Some(mut inv) => {
            inv.delete(client.as_ref()).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}

pub async fn update_project_invite(
    client: Arc<dyn welds::Client>,
    id: String,
    dto: ProjectInviteUpdateDto,
) -> anyhow::Result<ProjectInvite> {
    let invite = ProjectInvite::find_by_id(client.as_ref(), id).await?;
    match invite {
        Some(mut inv) => {
            inv.expire_at = dto.expire_at;
            inv.save(client.as_ref()).await?;

            Ok(inv.into_inner())
        }
        None => anyhow::bail!("not found project invite"),
    }
}

fn build_project_invite_query(filter: ProjectInviteFilterDto) -> welds::query::builder::QueryBuilder<ProjectInvite> {
    let mut query = ProjectInvite::all();
    if let Some(id) = filter.id {
        query = query.where_col(|c| c.id.equal(id.clone()));
    }
    if let Some(project_id) = filter.project_id {
        query = query.where_col(|c| c.project_id.equal(project_id.clone()));
    }
    if let Some(email) = filter.email {
        query = query.where_col(|c| c.email.equal(email.clone()));
    }

    query
}
