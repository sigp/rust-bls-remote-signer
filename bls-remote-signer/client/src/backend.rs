use crate::api_error::ApiError;
use crate::rest_api::Context;
use client_backend::PublicKeys;
use hyper::Request;
use std::sync::Arc;

/// HTTP handler to get the list of public keys in the backend.
pub fn get_public_keys<T>(_: T, ctx: Arc<Context>) -> Result<PublicKeys, ApiError> {
    let backend = ctx.backend.clone();

    backend
        .get_public_keys()
        .map_err(|e| ApiError::ServerError(format!("{}", e)))
}

/// HTTP handler to sign a message with the requested key.
pub fn sign_message(_req: Request<Vec<u8>>, _ctx: Arc<Context>) -> Result<String, ApiError> {
    // TODO
    // Implement. Stop copying the Lighthouse boilerplates, Herman.

    /*

    let public_key = path_segments.next().unwrap();
    let backend = ctx.backend.clone();

    // TODO
    // This backend.sign() is a PLACEHOLDER
    // We must
    // a. Use a proper `Handler`
    // b. Send the `signingRoot` to sign as well
    // c. Send a proper JSON payload as response
    //
    // TODO
    // Should we manage the `public_key` as &str or &[u8] ?
    match backend.sign(public_key) {
        Err(BackendError::ParameterIsNotAPublicKey(param)) => Err(ApiError::BadRequest(
            format!("Parameter is not a public key: {}", param),
        )),

        Err(BackendError::KeyNotFound(pk)) => Err(ApiError::BadRequest(format!(
            "Public key not found: {}",
            pk
        ))),

        Ok(pk) => Err(ApiError::NotImplemented(format!(
            "Not Implemented (You sent {})",
            pk
        ))),
    }

    */

    Ok("PLACEHOLDER PARA sign_message`".to_string())
}
