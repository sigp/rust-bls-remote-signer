mod error;
mod storage;
mod storage_dummy;
mod storage_raw_dir;

use clap::ArgMatches;
use error::BackendError;
use slog::{info, Logger};
pub use storage::PublicKeys;
use storage::Storage;
use storage_dummy::StorageDummy;
use storage_raw_dir::StorageRawDir;

/// A backend to be used by the Remote Signer HTTP API.
///
/// Designed to support several types of storages.
#[derive(Clone)]
pub struct Backend {
    storage: Box<dyn Storage>,
}

impl Backend {
    /// Creates a Backend with the given storage type.
    pub fn new(cli_args: &ArgMatches<'_>, log: &Logger) -> Result<Backend, String> {
        // Storage types are mutually exclusive.
        if let Some(path) = cli_args.value_of("storage-raw-dir") {
            info!(
                log,
                "Loading Backend";
                "storage type" => "raw dir",
                "directory" => path
            );

            StorageRawDir::new()
                .map(|storage| Backend {
                    storage: Box::new(storage),
                })
                .map_err(|e| format!("Storage Raw Dir: {}", e))
        } else if cli_args.is_present("storage-dummy") {
            info!(
                log,
                "Loading Backend";
                "storage type" => "dummy",
                "params?" => "no params"
            );

            StorageDummy::new()
                .map(|storage| Backend {
                    storage: Box::new(storage),
                })
                .map_err(|e| format!("Storage Dummy: {}", e))
        } else {
            Err("No backend supplied.".to_string())
        }
    }

    /// Returns the available public keys in the storage.
    pub fn get_public_keys(self) -> Result<PublicKeys, BackendError> {
        self.storage
            .get_public_keys()
            .map(|keys| PublicKeys { public_keys: keys })
    }

    /// Signs the message with the requested key in storage.
    pub fn sign_message(self, input: &str) -> Result<Vec<u8>, BackendError> {
        // TODO
        // This is placeholder logic.
        // The rough algorithm is:
        // - Are we being asked for a legit BLS public key?
        // - Do we have that key?
        // - Is the `signingRoot` legit?
        // - Perform the signature
        // - Get rid of the key (zeroize)
        // - Return the signed message
        let pk = self.storage.get_private_key(input)?;
        Ok(pk.to_vec())
    }
}
