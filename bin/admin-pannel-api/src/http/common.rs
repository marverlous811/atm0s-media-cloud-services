use poem::{http::StatusCode, IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ListQuery {
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize)]
pub struct ListResponse<E>
where
    E: Serialize,
{
    pub items: Vec<E>,
    pub total: usize,
}

pub fn to_response<E: Serialize>(response: anyhow::Result<E>) -> impl IntoResponse {
    match response {
        Ok(res) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(serde_json::to_vec(&res).expect("should convert to json")),
        Err(err) => Response::builder()
            .header("content-type", "plain-text")
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string()),
    }
}

#[allow(unused)]
pub fn to_response_list<E: Serialize>(response: anyhow::Result<(Vec<E>, usize)>) -> impl IntoResponse {
    match response {
        Ok((res, total)) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .header("X-Total-Count", total)
            .body(serde_json::to_vec(&ListResponse { items: res, total }).expect("should convert to json")),
        Err(err) => Response::builder()
            .header("content-type", "plain-text")
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string()),
    }
}
