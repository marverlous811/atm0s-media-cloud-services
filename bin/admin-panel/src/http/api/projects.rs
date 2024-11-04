use poem::{
    get, handler, post,
    web::{Data, Json},
    EndpointExt, IntoResponse, Route,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        models::project::Project,
        repositories::project::{create_project, get_projects, CreateProjectDto, ProjectFilterDto},
    },
    http::{
        middleware::{self, auth::UserId},
        HttpContext,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateProjectBody {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SyncProjectData {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Serialize)]
pub struct SyncProjectResponse {
    apps: Vec<SyncProjectData>,
}

#[handler]
pub async fn new_project(
    data: Data<&HttpContext>,
    body: Json<CreateProjectBody>,
    user_id: UserId,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        body: Json<CreateProjectBody>,
        user_id: UserId,
    ) -> anyhow::Result<Project> {
        let secret = atm0s_media_cloud_utils::string::generate_api_key(32);
        let project = create_project(
            data.db.clone(),
            CreateProjectDto {
                name: body.name.clone(),
                secret,
                owner: user_id.into(),
                options: Some(Default::default()),
                codecs: Some(Default::default()),
            },
        )
        .await?;
        Ok(project)
    }

    http_common::response::to_response(process(data, body, user_id).await)
}

#[handler]
pub async fn sync_projects(data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>) -> anyhow::Result<SyncProjectResponse> {
        let projects = get_projects(
            data.db.clone(),
            ProjectFilterDto {
                owner: None,
                name: None,
            },
            None,
            None,
        )
        .await?;
        Ok(SyncProjectResponse {
            apps: projects
                .into_iter()
                .map(|p| SyncProjectData {
                    app_id: p.id.clone(),
                    app_secret: p.secret.clone(),
                })
                .collect(),
        })
    }

    http_common::response::to_response(process(data).await)
}

pub fn build_project_route() -> Route {
    Route::new().nest("", post(new_project))
}

pub fn build_route(ctx: HttpContext) -> Route {
    Route::new()
        .nest(
            "/",
            build_project_route().with(middleware::auth::AuthMiddleware::new(ctx.clone())),
        )
        .nest(
            "/sync",
            get(sync_projects).with(middleware::api_key_auth::ApiKeyAuthMiddleware::new(ctx.clone())),
        )
}
