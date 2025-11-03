use ntex::http::StatusCode;
use ntex::web::{HttpResponse, WebResponseError};
use serde::Serialize;
use thiserror::Error;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: ErrorResponse) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Bad request: {message}")]
    BadRequest { message: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },

    #[error("Internal server error")]
    InternalServerError,

    #[error("Repository error: {message}")]
    RepositoryError { message: String },

    #[error("Conflict: {message}")]
    Conflict { message: String },
}

impl ApiError {
    pub fn not_found(resource: &str) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest {
            message: message.to_string(),
        }
    }

    pub fn validation_error(message: &str) -> Self {
        Self::ValidationError {
            message: message.to_string(),
        }
    }

    pub fn repository_error(message: &str) -> Self {
        Self::RepositoryError {
            message: message.to_string(),
        }
    }

    pub fn conflict(message: &str) -> Self {
        Self::Conflict {
            message: message.to_string(),
        }
    }

    fn to_error_response(&self) -> ErrorResponse {
        match self {
            ApiError::NotFound { resource } => ErrorResponse {
                code: "NOT_FOUND".to_string(),
                message: format!("Resource not found: {}", resource),
                details: None,
            },
            ApiError::BadRequest { message } => ErrorResponse {
                code: "BAD_REQUEST".to_string(),
                message: message.clone(),
                details: None,
            },
            ApiError::ValidationError { message } => ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: message.clone(),
                details: None,
            },
            ApiError::InternalServerError => ErrorResponse {
                code: "INTERNAL_SERVER_ERROR".to_string(),
                message: "An internal server error occurred".to_string(),
                details: None,
            },
            ApiError::RepositoryError { message } => ErrorResponse {
                code: "REPOSITORY_ERROR".to_string(),
                message: message.clone(),
                details: None,
            },
            ApiError::Conflict { message } => ErrorResponse {
                code: "CONFLICT".to_string(),
                message: message.clone(),
                details: None,
            },
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::RepositoryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Conflict { .. } => StatusCode::CONFLICT,
        }
    }
}

impl WebResponseError for ApiError {
    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        let error_response = self.to_error_response();
        HttpResponse::build(self.status_code()).json(&error_response)
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        ApiError::RepositoryError { message }
    }
}
