use poem::{http::StatusCode, IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListResponse<E>
where
    E: Serialize,
{
    pub items: Vec<E>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub fn to_response<E: Serialize>(response: anyhow::Result<E>) -> impl IntoResponse {
    match response {
        Ok(res) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(serde_json::to_vec(&res).expect("should convert to json")),
        Err(err) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .status(StatusCode::BAD_REQUEST)
            .body(
                serde_json::to_vec(&ErrorResponse {
                    message: err.to_string(),
                })
                .expect("should convert to json"),
            ),
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
            .header("content-type", "application/json; charset=utf-8")
            .status(StatusCode::BAD_REQUEST)
            .body(
                serde_json::to_vec(&ErrorResponse {
                    message: err.to_string(),
                })
                .expect("should convert to json"),
            ),
    }
}
