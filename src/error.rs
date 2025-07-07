use serde::{Deserialize, Serialize};
use worker::{Error, Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: u32,
    pub message: String,
}

impl ApiError {
    pub fn new(code: u32, message: impl Into<String>) -> Self {
        Self {
            error: code,
            message: message.into(),
        }
    }

    pub fn invalid_parameters(details: impl Into<String>) -> Self {
        Self::new(6, format!("Invalid parameters - {}", details.into()))
    }

    pub fn invalid_api_key() -> Self {
        Self::new(
            10,
            "Invalid API key - You must be granted a valid key by last.fm",
        )
    }

    pub fn service_offline() -> Self {
        Self::new(
            11,
            "Service Offline - This service is temporarily offline. Try again later.",
        )
    }

    pub fn invalid_signature() -> Self {
        Self::new(13, "Invalid method signature supplied")
    }

    pub fn temporary_error() -> Self {
        Self::new(
            16,
            "There was a temporary error processing your request. Please try again.",
        )
    }

    pub fn rate_limit_exceeded() -> Self {
        Self::new(
            29,
            "Rate limit exceeded - Your IP has made too many requests in a short period",
        )
    }

    pub fn to_response(&self) -> Result<Response, Error> {
        Response::ok(serde_json::to_string(self).unwrap()).map(|mut resp| {
            resp.headers_mut()
                .set("Content-Type", "application/json")
                .unwrap();
            resp
        })
    }
}

impl From<ApiError> for Response {
    fn from(error: ApiError) -> Self {
        error
            .to_response()
            .unwrap_or_else(|_| Response::error("Internal Server Error", 500).unwrap())
    }
}

impl From<worker::Error> for ApiError {
    fn from(_err: worker::Error) -> Self {
        ApiError::temporary_error()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
