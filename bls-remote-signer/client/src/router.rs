use crate::{api_error::ApiError, config::Config};
use environment::TaskExecutor;
use hyper::{Body, Method, Request, Response};
use slog::debug;
use std::sync::Arc;
use std::time::Instant;

pub struct Context {
    pub config: Config,
    pub executor: TaskExecutor,
    pub log: slog::Logger,
}

pub async fn on_http_request(
    req: Request<Body>,
    ctx: Arc<Context>,
) -> Result<Response<Body>, ApiError> {
    let path = req.uri().path().to_string();

    let received_instant = Instant::now();
    let log = ctx.log.clone();

    match route(req, ctx).await {
        Ok(response) => {
            debug!(
                log,
                "HTTP API request successful";
                "path" => path,
                "duration_ms" => Instant::now().duration_since(received_instant).as_millis()
            );
            Ok(response)
        }

        Err(error) => {
            debug!(
                log,
                "HTTP API request failure";
                "path" => path,
                "duration_ms" => Instant::now().duration_since(received_instant).as_millis()
            );
            Ok(error.into())
        }
    }
}

async fn route(req: Request<Body>, _ctx: Arc<Context>) -> Result<Response<Body>, ApiError> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    match (method, path.as_ref()) {
        (Method::GET, "/upcheck") => Ok(Response::new(Body::from("OK"))),
        (Method::GET, "/publicKeys") => {
            Err(ApiError::NotImplemented("Not Implemented".to_string()))
        }
        (Method::POST, _) => route_post(path.as_ref()),
        _ => Err(ApiError::NotFound(
            "Request path and/or method not found.".to_string(),
        )),
    }
}

fn route_post(path: &str) -> Result<Response<Body>, ApiError> {
    let mut path_segments = path[1..].trim_end_matches('/').split('/');

    let first_segment = match path_segments.next() {
        Some("sign") => "sign",
        _ => "",
    };

    if first_segment == "" {
        return Err(ApiError::NotFound(
            "Request path and/or method not found.".to_string(),
        ));
    }

    let path_segments_count = path_segments.clone().count();

    if path_segments_count == 0 {
        return Err(ApiError::BadRequest(
            "Parameter 'public-key' needed in route /sign/{public-key}".to_string(),
        ));
    }

    if path_segments_count > 1 {
        return Err(ApiError::BadRequest(
            "Only one segment is allowed after /sign".to_string(),
        ));
    }

    match backend_sign(path_segments.next().unwrap()) {
        Err(BackendError::ParameterIsNotAPublicKey(param)) => Err(ApiError::BadRequest(format!(
            "Parameter is not a public key: {}",
            param
        ))),
        Err(BackendError::KeyNotFound(pk)) => Err(ApiError::BadRequest(format!(
            "Public key not found: {}",
            pk
        ))),
        Ok(pk) => Err(ApiError::NotImplemented(format!(
            "Not Implemented (You sent {})",
            pk
        ))),
    }
}

// PLACEHOLDER. Should go to its own crate.
#[allow(dead_code)]
enum BackendError {
    ParameterIsNotAPublicKey(String),
    KeyNotFound(String),
}

// PLACEHOLDER. Should go to its own crate.
fn backend_sign(public_key: &str) -> Result<String, BackendError> {
    Ok(public_key.to_string())
}
