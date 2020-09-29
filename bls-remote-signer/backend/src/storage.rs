use crate::BackendError;

/// The storage medium for the secret keys used by a `Backend`.
pub trait Storage: Send + Sync {
    /// Queries storage for the available keys to sign.
    fn get_public_keys(self: Box<Self>) -> Result<Vec<String>, BackendError>;

    /// Retrieves secret key from storage, using its public key as reference.
    /// While (at the moment) it is practical to work with `String` as
    /// the returning value, we may want to consider a different type when other
    /// storage mediums are supported by this signer.
    fn get_secret_key(self: Box<Self>, input: &str) -> Result<String, BackendError>;

    /// Needed for the Backend struct to implement Clone
    fn box_clone(&self) -> Box<dyn Storage>;
}

impl Clone for Box<dyn Storage> {
    /// Needed for the Backend struct to implement Clone
    fn clone(&self) -> Box<dyn Storage> {
        self.box_clone()
    }
}
