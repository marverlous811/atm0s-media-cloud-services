use std::sync::Arc;

use crate::database::models::project_member::{MemberRole, ProjectMember};

#[derive(Debug, Clone)]
pub struct CreateProjectMemberDto {
    pub project_id: String,
    pub user_id: String,
    pub role: MemberRole,
}

pub async fn create_project_member(
    client: Arc<dyn welds::Client>,
    dto: CreateProjectMemberDto,
) -> anyhow::Result<ProjectMember> {
    let mut member = ProjectMember::new();
    member.project_id = dto.project_id;
    member.user_id = dto.user_id;
    member.role = dto.role.to_string();

    member.save(client.as_ref()).await?;
    Ok(member.into_inner())
}
