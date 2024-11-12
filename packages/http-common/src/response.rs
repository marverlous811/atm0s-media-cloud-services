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

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: bool,
}

pub fn to_response<E: Serialize>(response: anyhow::Result<E>) -> impl IntoResponse {
    match response {
        Ok(res) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(serde_json::to_vec(&res).expect("should convert to json")),
        Err(err) => to_response_error(err),
    }
}

#[allow(unused)]
pub fn to_response_list<E: Serialize>(response: anyhow::Result<(Vec<E>, usize)>) -> impl IntoResponse {
    match response {
        Ok((res, total)) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .header("X-Total-Count", total)
            .body(serde_json::to_vec(&ListResponse { items: res, total }).expect("should convert to json")),
        Err(err) => to_response_error(err),
    }
}

pub fn to_response_error(err: anyhow::Error) -> Response {
    log::error!("Error: {:?}", err);
    let poem_err = err.downcast_ref::<poem::Error>();
    if let Some(poem_err) = poem_err {
        return Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .status(poem_err.status())
            .body(
                serde_json::to_vec(&ErrorResponse {
                    message: poem_err.to_string(),
                })
                .expect("should convert to json"),
            );
    }

    let serde_err = err.downcast_ref::<serde_json::Error>();
    if let Some(serde_err) = serde_err {
        return Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .status(StatusCode::BAD_REQUEST)
            .body(
                serde_json::to_vec(&ErrorResponse {
                    message: serde_err.to_string(),
                })
                .expect("should convert to json"),
            );
    }

    Response::builder()
        .header("content-type", "application/json; charset=utf-8")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(
            serde_json::to_vec(&ErrorResponse {
                message: err.to_string(),
            })
            .expect("should convert to json"),
        )
}
