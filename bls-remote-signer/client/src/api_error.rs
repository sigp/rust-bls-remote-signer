use hyper::{Body, Response, StatusCode};
use std::error::Error as StdError;

#[derive(PartialEq, Debug, Clone)]
pub enum ApiError {
    ServerError(String),
    NotImplemented(String),
    BadRequest(String),
    NotFound(String),
}

pub type ApiResult = Result<Response<Body>, ApiError>;

impl ApiError {
    pub fn status_code(self) -> (StatusCode, String) {
        match self {
            ApiError::ServerError(desc) => (StatusCode::INTERNAL_SERVER_ERROR, desc),
            ApiError::NotImplemented(desc) => (StatusCode::NOT_IMPLEMENTED, desc),
            ApiError::BadRequest(desc) => (StatusCode::BAD_REQUEST, desc),
            ApiError::NotFound(desc) => (StatusCode::NOT_FOUND, desc),
        }
    }
}

impl Into<Response<Body>> for ApiError {
    fn into(self) -> Response<Body> {
        let (status_code, desc) = self.status_code();
        Response::builder()
            .status(status_code)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Body::from(desc))
            .expect("Response should always be created.")
    }
}

impl From<hyper::error::Error> for ApiError {
    fn from(e: hyper::error::Error) -> ApiError {
        ApiError::ServerError(format!("Networking error: {:?}", e))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> ApiError {
        ApiError::ServerError(format!("IO error: {:?}", e))
    }
}

impl StdError for ApiError {
    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let status = self.clone().status_code();
        write!(f, "{:?}: {:?}", status.0, status.1)
    }
}
