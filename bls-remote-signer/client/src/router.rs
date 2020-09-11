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
        _ => Err(ApiError::NotFound(
            "Request path and/or method not found.".to_owned(),
        )),
    }
}
