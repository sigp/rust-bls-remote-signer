#[derive(Debug)]
pub enum BackendError {
    StorageError(String),
    InvalidPublicKey(String),
    InvalidPrivateKey(String),
    InvalidSigningRoot(String),
    KeyNotFound(String),
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BackendError::StorageError(e) => write!(f, "Storage: {}", e),
            BackendError::InvalidPublicKey(e) => write!(f, "Invalid Public Key: {}", e),
            BackendError::InvalidPrivateKey(e) => write!(f, "Invalid Private Key: {}", e),
            BackendError::InvalidSigningRoot(e) => write!(f, "Invalid Signing Root: {}", e),
            BackendError::KeyNotFound(e) => write!(f, "Key Not Found: {}", e),
        }
    }
}

impl From<std::io::Error> for BackendError {
    fn from(e: std::io::Error) -> BackendError {
        BackendError::StorageError(format!("{}", e))
    }
}
