use poem::{
    handler,
    web::{Data, Query},
    IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::repositories::project::{get_projects, ProjectFilterDto},
    http::HttpContext,
};

#[derive(Debug, Deserialize)]
pub struct SyncProjectQuery {
    pub secret: String,
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

/// Sync projects from database to other services
/// This validate with query secret
#[handler]
pub async fn sync_projects(query: Query<SyncProjectQuery>, data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, query: SyncProjectQuery) -> anyhow::Result<SyncProjectResponse> {
        if query.secret != data.cfg.cluster_secret {
            return Err(anyhow::anyhow!("Invalid secret"));
        }

        let projects = get_projects(
            data.db.clone(),
            ProjectFilterDto {
                id: None,
                owner: None,
                name: None,
                user_id: None,
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

    http_common::response::to_response(process(data, query.0).await)
}
