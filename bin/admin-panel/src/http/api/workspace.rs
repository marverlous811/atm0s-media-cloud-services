mod projects;

use poem::{
    get, handler, post, put,
    web::{Data, Json, Path},
    IntoResponse, Route,
};
use projects::{list_projects, new_project, project_detail, project_update};
use serde::Deserialize;

use crate::{
    database::{
        models::workspace::Workspace,
        repositories::workspace::{
            count_workspaces, create_workspace, get_workspaces, update_workspace, CreateWorkspaceDto,
            UpdateWorkspaceDto, WorkspaceFilterDto,
        },
    },
    http::{middleware::clerk_auth::ClerkUserId, HttpContext},
};

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceBody {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceBody {
    pub name: Option<String>,
}

#[handler]
pub async fn new_workspace(
    data: Data<&HttpContext>,
    body: Json<CreateWorkspaceBody>,
    user_id: ClerkUserId,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        body: Json<CreateWorkspaceBody>,
        user_id: String,
    ) -> anyhow::Result<Workspace> {
        let workspace = create_workspace(
            data.db.clone(),
            CreateWorkspaceDto {
                name: body.name.clone(),
                owner: user_id.clone(),
            },
        )
        .await?;

        Ok(workspace)
    }

    http_common::response::to_response(process(data, body, user_id.into()).await)
}

#[handler]
pub async fn list_workspace(data: Data<&HttpContext>, user_id: ClerkUserId) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, _user_id: String) -> anyhow::Result<(Vec<Workspace>, usize, usize)> {
        let filter = WorkspaceFilterDto::default();
        let workspaces = get_workspaces(data.db.clone(), filter.clone(), None, None).await?;
        let count = count_workspaces(data.db.clone(), filter.clone()).await?;
        Ok((workspaces, 0, count as usize))
    }

    http_common::response::to_response_list(process(data, user_id.into()).await)
}

#[handler]
pub async fn change_workspace(
    data: Data<&HttpContext>,
    Path(workspace_id): Path<String>,
    body: Json<UpdateWorkspaceBody>,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        workspace_id: String,
        body: Json<UpdateWorkspaceBody>,
    ) -> anyhow::Result<Workspace> {
        let workspace = update_workspace(
            data.db.clone(),
            workspace_id,
            UpdateWorkspaceDto {
                name: body.name.clone(),
            },
        )
        .await?;

        Ok(workspace)
    }

    http_common::response::to_response(process(data, workspace_id, body).await)
}

pub fn build_route() -> Route {
    Route::new()
        .at("/:workspace_id/projects", post(new_project).get(list_projects))
        .at(
            "/:workspace_id/projects/:project_id",
            get(project_detail).put(project_update),
        )
        .at("/:workspace_id", put(change_workspace))
        .at("", post(new_workspace).get(list_workspace))
}
