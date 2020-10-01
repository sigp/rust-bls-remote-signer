use crate::api_error::{ApiError, ApiResult};
use crate::rest_api::Context;
use hyper::{Body, Request, Response, StatusCode};
use serde::Serialize;
use std::sync::Arc;

/// Provides a HTTP request handler with specific functionality.
pub struct Handler<T: Send + Sync> {
    req: Request<()>,
    body: Body,
    ctx: Arc<Context<T>>,
    allow_body: bool,
}
// <T: Clone + Send + Sync + 'static>
impl<T: 'static + Send + Sync> Handler<T> {
    /// Start handling a new request.
    pub fn new(req: Request<Body>, ctx: Arc<Context<T>>) -> Result<Self, ApiError> {
        let (req_parts, body) = req.into_parts();
        let req = Request::from_parts(req_parts, ());

        Ok(Self {
            req,
            body,
            ctx,
            allow_body: false,
        })
    }

    /// Return a simple static value.
    ///
    /// Does not use the blocking executor.
    pub async fn static_value<V>(self, value: V) -> Result<HandledRequest<V>, ApiError> {
        // Always check and disallow a body for a static value.
        let _ = Self::get_body(self.body, false).await?;

        Ok(HandledRequest { value })
    }

    /// The default behaviour is to return an error if any body is supplied in the request. Calling
    /// this function disables that error.
    pub fn allow_body(mut self) -> Self {
        self.allow_body = true;
        self
    }

    /// Spawns `func` on the blocking executor.
    ///
    /// This method is suitable for handling long-running or intensive tasks.
    pub async fn in_blocking_task<F, V>(self, func: F) -> Result<HandledRequest<V>, ApiError>
    where
        V: Send + Sync + 'static,
        F: Fn(Request<Vec<u8>>, Arc<Context<T>>) -> Result<V, ApiError> + Send + Sync + 'static,
    {
        let ctx = self.ctx;
        let executor = ctx.executor.clone();
        let body = Self::get_body(self.body, self.allow_body).await?;
        let (req_parts, _) = self.req.into_parts();
        let req = Request::from_parts(req_parts, body);

        let value = executor
            .clone()
            .handle
            .spawn_blocking(move || func(req, ctx))
            .await
            .map_err(|e| {
                ApiError::ServerError(format!(
                    "Failed to get blocking join handle: {}",
                    e.to_string()
                ))
            })??;

        Ok(HandledRequest { value })
    }

    /// Downloads the bytes for `body`.
    async fn get_body(body: Body, allow_body: bool) -> Result<Vec<u8>, ApiError> {
        let bytes = hyper::body::to_bytes(body)
            .await
            .map_err(|e| ApiError::ServerError(format!("Unable to get request body: {:?}", e)))?;

        if !allow_body && !bytes[..].is_empty() {
            Err(ApiError::BadRequest(
                "The request body must be empty".to_string(),
            ))
        } else {
            Ok(bytes.into_iter().collect())
        }
    }
}

/// A request that has been "handled" and now a result (`value`) needs to be serialized and
/// returned.
pub struct HandledRequest<V> {
    value: V,
}

impl<V: Serialize> HandledRequest<V> {
    /// Suitable for items which only implement `serde`.
    pub fn serde_encodings(self) -> ApiResult {
        let body = Body::from(serde_json::to_string(&self.value).map_err(|e| {
            ApiError::ServerError(format!(
                "Unable to serialize response body as JSON: {:?}",
                e
            ))
        })?);

        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(body)
            .map_err(|e| ApiError::ServerError(format!("Failed to build response: {:?}", e)))
    }
}
