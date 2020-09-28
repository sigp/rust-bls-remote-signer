use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UpcheckApiResponse {
    pub status: String,
}

/// Contains the response to the `get_public_keys` API.
#[derive(Deserialize, Serialize)]
pub struct PublicKeysApiResponse {
    pub public_keys: Vec<String>,
}

/// Contains the response to the `sign_message` API.
#[derive(Deserialize, Serialize)]
pub struct SignatureApiResponse {
    pub signature: String,
}
