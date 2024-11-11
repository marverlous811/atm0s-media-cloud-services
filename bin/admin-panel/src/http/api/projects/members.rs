use std::fmt;

use poem::{
    handler,
    web::{Data, Json, Path},
    IntoResponse,
};
use serde::Deserialize;

use crate::{
    database::{
        models::project_member::ProjectInvite,
        repositories::project_invite::{
            create_project_invite, get_project_invite, update_project_invite, CreateProjectInviteDto,
            ProjectInviteFilterDto, ProjectInviteUpdateDto,
        },
    },
    http::HttpContext,
};

#[derive(Debug, Deserialize)]
pub enum MemberRole {
    #[serde(rename = "admin")]
    ADMIN,
    #[serde(rename = "member")]
    MEMBER,
}

impl fmt::Display for MemberRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Deserialize)]
pub struct ProjectInviteBody {
    email: String,
    role: MemberRole,
}

#[handler]
pub async fn invite(
    data: Data<&HttpContext>,
    Path(project_id): Path<String>,
    body: Json<ProjectInviteBody>,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        project_id: String,
        body: Json<ProjectInviteBody>,
    ) -> anyhow::Result<ProjectInvite> {
        let invite = get_project_invite(
            data.db.clone(),
            ProjectInviteFilterDto {
                email: Some(body.email.clone()),
                project_id: Some(project_id.clone()),
                id: None,
            },
        )
        .await?;

        log::info!("hello invite");
        match invite {
            Some(invite) => {
                update_project_invite(
                    data.db.clone(),
                    invite.id,
                    ProjectInviteUpdateDto {
                        expire_at: (chrono::Utc::now().timestamp_millis() + 2 * 24 * 60 * 60 * 1000) as i64,
                    },
                )
                .await
            }
            None => {
                create_project_invite(
                    data.db.clone(),
                    CreateProjectInviteDto {
                        project_id: project_id.clone(),
                        email: body.email.clone(),
                        role: body.role.to_string(),
                        expire_at: (chrono::Utc::now().timestamp_millis() + 2 * 24 * 60 * 60 * 1000) as i64,
                    },
                )
                .await
            }
        }
    }

    http_common::response::to_response(process(data, project_id, body).await)
}
