use crate::BackendError;
use serde::Serialize;

/// The storage medium for the private keys used by a `Backend`.
pub trait Storage: Send + Sync {
    // TODO
    // - Should return a struct instead of a Vec<String>.
    // - Function documentation.
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

#[derive(Serialize)]
pub struct PublicKeys {
    pub public_keys: Vec<String>,
}
