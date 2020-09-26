use crate::api_error::ApiError;
use crate::rest_api::Context;
use client_backend::PublicKeys;
use hyper::Request;
use std::sync::Arc;

/// HTTP handler to get the list of public keys in the backend.
pub fn get_public_keys<T>(_: T, ctx: Arc<Context>) -> Result<PublicKeys, ApiError> {
    let backend = ctx.backend.clone();

    // TODO
    // Consider to give a status code 404 and an empty payload in the "no keys" use case.
    backend
        .get_public_keys()
        .map_err(|e| ApiError::ServerError(format!("{}", e)))
}

/// HTTP handler to sign a message with the requested key.
pub fn sign_message(_req: Request<Vec<u8>>, _ctx: Arc<Context>) -> Result<String, ApiError> {
    todo!()

    /*
    - Se necesita validar que este el signingRoot, sino ApiError
    - Hacer la llamada
    - Encapsular la respuesta
    */
}
