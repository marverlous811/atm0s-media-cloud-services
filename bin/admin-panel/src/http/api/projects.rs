mod members;

use http_common::response::StatusResponse;
use members::invite;
use poem::{
    handler, post, put,
    web::{Data, Json, Path},
    Error, IntoResponse, Route,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    database::{
        models::{
            project::{Project, ProjectCodecs, ProjectOptions},
            project_member::MemberRole,
        },
        repositories::{
            project::{
                count_projects, create_project, delete_project, get_project, get_projects, update_project,
                CreateProjectDto, ProjectFilterDto, UpdateProjectDto,
            },
            project_member::{create_project_member, CreateProjectMemberDto},
        },
    },
    http::{middleware::clerk_auth::ClerkUserId, HttpContext},
};

#[derive(Debug, Deserialize)]
pub struct CreateProjectBody {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectBody {
    pub name: Option<String>,
    pub options: Option<ProjectOptions>,
    pub codecs: Option<ProjectCodecs>,
}

#[handler]
pub async fn new_project(
    data: Data<&HttpContext>,
    body: Json<CreateProjectBody>,
    user_id: ClerkUserId,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        body: Json<CreateProjectBody>,
        user_id: String,
    ) -> anyhow::Result<Project> {
        let secret = atm0s_media_cloud_utils::string::generate_api_key(32);
        let project = create_project(
            data.db.clone(),
            CreateProjectDto {
                name: body.name.clone(),
                secret,
                owner: user_id.clone(),
                options: Some(Default::default()),
                codecs: Some(Default::default()),
            },
        )
        .await?;

        create_project_member(
            data.db.clone(),
            CreateProjectMemberDto {
                user_id: user_id.clone(),
                project_id: project.id.clone(),
                role: MemberRole::OWNER,
            },
        )
        .await?;
        Ok(project)
    }

    http_common::response::to_response(process(data, body, user_id.into()).await)
}

#[handler]
pub async fn list_projects(data: Data<&HttpContext>, user_id: ClerkUserId) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, user_id: String) -> anyhow::Result<(Vec<Project>, usize, usize)> {
        let filter = ProjectFilterDto {
            id: None,
            user_id: Some(user_id),
            owner: None,
            name: None,
        };
        let projects = get_projects(data.db.clone(), filter.clone(), None, Some(0)).await?;
        let count = count_projects(data.db.clone(), filter.clone()).await?;

        Ok((projects, 0, count as usize))
    }

    http_common::response::to_response_list(process(data, user_id.into()).await)
}

#[handler]
pub async fn project_detail(
    data: Data<&HttpContext>,
    user_id: ClerkUserId,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, user_id: String, project_id: String) -> anyhow::Result<Project> {
        match get_project(
            data.db.clone(),
            ProjectFilterDto {
                user_id: Some(user_id.clone()),
                id: Some(project_id.clone()),
                owner: None,
                name: None,
            },
        )
        .await?
        {
            Some(project) => Ok(project),
            None => anyhow::bail!(Error::from_string("project not found", StatusCode::NOT_FOUND)),
        }
    }

    http_common::response::to_response(process(data, user_id.into(), project_id).await)
}

#[handler]
pub async fn project_update(
    data: Data<&HttpContext>,
    Path(project_id): Path<String>,
    body: Json<UpdateProjectBody>,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        project_id: String,
        body: Json<UpdateProjectBody>,
    ) -> anyhow::Result<Project> {
        update_project(
            data.db.clone(),
            project_id,
            UpdateProjectDto {
                name: body.name.clone(),
                options: body.options.clone(),
                codecs: body.codecs.clone(),
            },
        )
        .await
    }

    http_common::response::to_response(process(data, project_id, body).await)
}

#[handler]
pub async fn project_delete(data: Data<&HttpContext>, Path(project_id): Path<String>) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, project_id: String) -> anyhow::Result<StatusResponse> {
        let status = delete_project(data.db.clone(), project_id).await?;
        Ok(StatusResponse { status })
    }

    http_common::response::to_response(process(data, project_id).await)
}

pub fn build_project_route() -> Route {
    Route::new()
        .at(
            "/:project_id",
            put(project_update).get(project_detail).delete(project_delete),
        )
        .at("/:project_id/members/invite", post(invite))
        .at("/", post(new_project).get(list_projects))
}

pub fn build_route() -> Route {
    Route::new().nest("", build_project_route())
}
