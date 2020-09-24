use crate::BackendError;
use serde::{Deserialize, Serialize};

/// The storage medium for the private keys used by a `Backend`.
pub trait Storage: Send + Sync {
    /// Queries storage for the available keys to sign.
    /// Backend consumes this function and either encapsulates the `Vec<String>`
    /// into a `PublicKeys` struct, or bubbles up the `BackendError`.
    fn get_public_keys(self: Box<Self>) -> Result<Vec<String>, BackendError>;

    // TODO
    // - Should account for zeroization.
    // - Regarding above, understand ownership of the returned value.
    // - Function documentation.
    fn get_private_key(self: Box<Self>, input: &str) -> Result<Vec<u8>, BackendError>;

    /// Needed for Backend to implement Clone
    fn box_clone(&self) -> Box<dyn Storage>;
}

impl Clone for Box<dyn Storage> {
    /// Needed for Backend to implement Clone
    fn clone(&self) -> Box<dyn Storage> {
        self.box_clone()
    }
}

/// Contains the response to the `get_public_keys` API.
#[derive(Deserialize, Serialize)]
pub struct PublicKeys {
    pub public_keys: Vec<String>,
}
