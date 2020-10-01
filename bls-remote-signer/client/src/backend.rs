use crate::api_error::ApiError;
use crate::api_response::{PublicKeysApiResponse, SignatureApiResponse};
use crate::rest_api::Context;
use client_backend::{BackendError, Storage};
use hyper::Request;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::sync::Arc;

lazy_static! {
    static ref PUBLIC_KEY_FROM_PATH_REGEX: Regex = Regex::new(r"^/[^/]+/([^/]*)").unwrap();
}

/// HTTP handler to get the list of public keys in the backend.
pub fn get_public_keys<T: Storage, U>(
    _: U,
    ctx: Arc<Context<T>>,
) -> Result<PublicKeysApiResponse, ApiError> {
    let public_keys = ctx
        .backend
        .get_public_keys()
        .map_err(|e| ApiError::ServerError(format!("{}", e)))?;

    if public_keys.is_empty() {
        return Err(ApiError::NotFound("No keys found in storage.".to_string()));
    }

    Ok(PublicKeysApiResponse { public_keys })
}

/// HTTP handler to sign a message with the requested key.
pub fn sign_message<T: Storage>(
    req: Request<Vec<u8>>,
    ctx: Arc<Context<T>>,
) -> Result<SignatureApiResponse, ApiError> {
    let body: Value = serde_json::from_slice(req.body())
        .map_err(|e| ApiError::BadRequest(format!("Unable to parse JSON: {:?}", e)))?;

    let signing_root = match &body["signingRoot"] {
        Value::String(signing_root) => Ok(signing_root.to_string()),

        Value::Null => Err(ApiError::BadRequest(
            "Missing field signingRoot.".to_string(),
        )),

        _ => Err(ApiError::BadRequest(format!(
            "Invalid field signingRoot: {}",
            body["signingRoot"].to_string()
        ))),
    }?;

    // The backend controls against empty signingRoot parameters.
    // We can save us some cpu cycles, though, if we catch this earlier.
    if signing_root == "" {
        return Err(ApiError::BadRequest("Empty field signingRoot.".to_string()));
    }

    // This public key parameter should have been validated by the router.
    // We are just going to extract it from the request.
    let path = req.uri().path().to_string();

    let rc = |path: &str| -> Result<String, String> {
        let caps = PUBLIC_KEY_FROM_PATH_REGEX.captures(path).ok_or("")?;
        let re_match = caps.get(1).ok_or("")?;
        Ok(re_match.as_str().to_string())
    };

    let public_key = rc(&path).map_err(|_| {
        ApiError::BadRequest(format!("Unable to get public key from path: {:?}", path))
    })?;

    match ctx.backend.clone().sign_message(&public_key, &signing_root) {
        Ok(signature) => Ok(SignatureApiResponse { signature }),

        Err(BackendError::KeyNotFound(_)) => {
            Err(ApiError::NotFound(format!("Key not found: {}", public_key)))
        }

        Err(BackendError::InvalidSigningRoot(_)) => Err(ApiError::BadRequest(format!(
            "Invalid signingRoot: {}",
            signing_root
        ))),

        Err(BackendError::InvalidPublicKey(_)) => Err(ApiError::BadRequest(format!(
            "Invalid public key: {}",
            public_key
        ))),

        // Catches InvalidSecretKey, KeyMismatch and StorageError.
        Err(e) => Err(ApiError::ServerError(e.to_string())),
    }
}
