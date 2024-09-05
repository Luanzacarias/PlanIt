use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Ok {
        status: String,
        message: String,
        data: Option<serde_json::Value>,
    },
    Created {
        status: String,
        message: String,
        data: Option<serde_json::Value>,
    },
    BadRequestError {
        status: String,
        message: String,
        errors: Option<serde_json::Value>,
    },
    Unauthorized {
        status: String,
        message: String,
    },
    UnprocessableEntity {
        status: String,
        message: String,
        errors: Option<serde_json::Value>,
    },
    ServerError {
        status: String,
        message: String,
        errors: Option<serde_json::Value>,
    },
    NotFound {
        status: String,
        message: String,
    },
}

impl ApiResponse {
    pub fn ok<T: Serialize>(message: &str, data: Option<T>) -> Self {
        ApiResponse::Ok {
            status: "success".to_string(),
            message: message.to_string(),
            data: data.map(|d| serde_json::to_value(d).unwrap()),
        }
    }

    pub fn created<T: Serialize>(message: &str, data: Option<T>) -> Self {
        ApiResponse::Created {
            status: "success".to_string(),
            message: message.to_string(),
            data: data.map(|d| serde_json::to_value(d).unwrap()),
        }
    }

    pub fn bad_request<T: Serialize>(message: &str, errors: Option<T>) -> Self {
        ApiResponse::BadRequestError {
            status: "error".to_string(),
            message: message.to_string(),
            errors: errors.map(|e| serde_json::to_value(e).unwrap()),
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        ApiResponse::Unauthorized {
            status: "error".to_string(),
            message: message.to_string(),
        }
    }

    pub fn unprocessable_entity<T: Serialize>(message: &str, errors: Option<T>) -> Self {
        ApiResponse::UnprocessableEntity {
            status: "error".to_string(),
            message: message.to_string(),
            errors: errors.map(|e| serde_json::to_value(e).unwrap()),
        }
    }

    pub fn server_error<T: Serialize>(message: Option<&str>, errors: Option<T>) -> Self {
        ApiResponse::ServerError {
            status: "error".to_string(),
            message: message.unwrap_or("Internal server error").to_string(),
            errors: errors.map(|e| serde_json::to_value(e).unwrap()),
        }
    }

    pub fn not_found(message: &str) -> Self {
        ApiResponse::NotFound {
            status: "error".to_string(),
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status_code: StatusCode = match &self {
            ApiResponse::Ok { .. } => StatusCode::OK,
            ApiResponse::Created { .. } => StatusCode::CREATED,
            ApiResponse::BadRequestError { .. } => StatusCode::BAD_REQUEST,
            ApiResponse::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ApiResponse::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ApiResponse::ServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiResponse::NotFound { .. } => StatusCode::NOT_FOUND,
        };

        let json_response = Json(self);
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            json_response,
        )
            .into_response()
    }
}
