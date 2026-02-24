use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

pub enum ApiError {
    NotFound(String),
    Conflict(String),
    Validation(String),
    Internal,
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {

        match self {
            ApiError::NotFound(msg) => {
                build_problem(
                    StatusCode::NOT_FOUND,
                    "Resource not found",
                    Some(msg),
                )
            },
            ApiError::Conflict(msg) => {
                build_problem(
                    StatusCode::CONFLICT,
                    "Conflict",
                    Some(msg),
                )
            },
            ApiError::Validation(msg) => {
                build_problem(
                    StatusCode::BAD_REQUEST,
                    "Validation error",
                    Some(msg),
                )
            },
            ApiError::Internal => {
                build_problem(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    None,
                )
            },
        }
    }
}

fn build_problem(
    status: StatusCode,
    title: &str,
    detail: Option<String>,
) -> Response {

    let problem = ProblemDetails {
        r#type: format!("https://httpstatuses.com/{}", status.as_u16()),
        title: title.to_string(),
        status: status.as_u16(),
        detail,
        instance: None,
    };

    (status, Json(problem)).into_response()
}

use serde::Serialize;

#[derive(Serialize)]
pub struct ProblemDetails {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: Option<String>,
    pub instance: Option<String>,
}