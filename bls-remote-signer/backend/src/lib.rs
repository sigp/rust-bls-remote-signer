mod error;
mod storage;
mod storage_raw_dir;

use clap::ArgMatches;
use error::BackendError;
use slog::{info, Logger};
pub use storage::PublicKeys;
use storage::Storage;
use storage_raw_dir::StorageRawDir;

/// A backend to be used by the Remote Signer HTTP API.
///
/// Designed to support several types of storages.
#[derive(Clone)]
pub struct Backend {
    storage: Box<dyn Storage>,
}

impl Backend {
    /// Creates a Backend with the given storage type at the CLI arguments.
    ///
    /// Storage types supported:
    /// - Raw files in directory
    ///   - `--storage-raw-dir <DIR>`
    pub fn new(cli_args: &ArgMatches<'_>, log: &Logger) -> Result<Backend, String> {
        // Storage types are mutually exclusive.
        if let Some(path) = cli_args.value_of("storage-raw-dir") {
            info!(
                log,
                "Loading Backend";
                "storage type" => "raw dir",
                "directory" => path
            );

            StorageRawDir::new(path)
                .map(|storage| Backend {
                    storage: Box::new(storage),
                })
                .map_err(|e| format!("Storage Raw Dir: {}", e))
        } else {
            Err("No backend supplied.".to_string())
        }
    }

    /// Returns the available public keys in storage.
    pub fn get_public_keys(self) -> Result<PublicKeys, BackendError> {
        self.storage
            .get_public_keys()
            .map(|keys| PublicKeys { public_keys: keys })
    }

    /// Signs the message with the requested key in storage.
    pub fn sign_message(self, input: &str) -> Result<Vec<u8>, BackendError> {
        let pk = self.storage.get_private_key(input)?;
        Ok(pk.to_vec())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use helpers::*;
    use sloggers::{null::NullLoggerBuilder, Build};
    use tempdir::TempDir;

    pub fn new_storage_with_tmp_dir() -> (Box<dyn Storage>, TempDir) {
        let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
        let storage = StorageRawDir::new(tmp_dir.path().to_str().unwrap()).unwrap();
        // The methods defined in the trait are available for this arbitraty self type.
        (Box::new(storage), tmp_dir)
    }

    fn get_null_logger() -> Logger {
        let log_builder = NullLoggerBuilder;
        log_builder.build().unwrap()
    }

    #[test]
    fn backend_new_no_backend_supplied() {
        let matches = set_matches(vec!["this_test"]);

        match Backend::new(&matches, &get_null_logger()) {
            Ok(_) => panic!("This invocation to Backend::new() should return error"),
            Err(e) => assert_eq!(e.to_string(), "No backend supplied.",),
        }
    }

    #[test]
    fn backend_new_storage_raw_dir_param_path_does_not_exist() {
        let matches = set_matches(vec!["this_test", "--storage-raw-dir", "/dev/null/foo"]);

        match Backend::new(&matches, &get_null_logger()) {
            Ok(_) => panic!("This invocation to Backend::new() should return error"),
            Err(e) => assert_eq!(e.to_string(), "Storage Raw Dir: Path does not exist.",),
        }
    }

    #[test]
    fn backend_new_storage_raw_dir_param_path_is_not_a_dir() {
        let matches = set_matches(vec!["this_test", "--storage-raw-dir", "/dev/null"]);

        match Backend::new(&matches, &get_null_logger()) {
            Ok(_) => panic!("This invocation to Backend::new() should return error"),
            Err(e) => assert_eq!(e.to_string(), "Storage Raw Dir: Path is not a directory.",),
        }
    }

    #[test]
    fn backend_new_storage_raw_dir_param_path_inaccessible() {
        let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
        set_permissions(tmp_dir.path(), 0o40311);

        let matches = set_matches(vec![
            "this_test",
            "--storage-raw-dir",
            tmp_dir.path().to_str().unwrap(),
        ]);

        // TODO
        // Pay attention to this error message --> Permission denied (os error 13)
        // We may want to switch to a regular expression strategy.
        match Backend::new(&matches, &get_null_logger()) {
            Ok(_) => panic!("This invocation to Backend::new() should return error"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Storage Raw Dir: Permission denied (os error 13)",
            ),
        }

        // A `d-wx--x--x` directory is innaccesible but not unwrittable.
        // By switching back to `drwxr-xr-x` we can get rid of the
        // temporal directory once we leave this scope.
        set_permissions(tmp_dir.path(), 0o40755);
    }

    #[test]
    fn backend_new_storage_raw_dir_is_accessible() {
        let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();

        let matches = set_matches(vec![
            "this_test",
            "--storage-raw-dir",
            tmp_dir.path().to_str().unwrap(),
        ]);

        match Backend::new(&matches, &get_null_logger()) {
            Ok(_) => (),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }

    #[test]
    fn backend_get_public_keys_raw_dir_empty_dir() {
        let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();

        let matches = set_matches(vec![
            "this_test",
            "--storage-raw-dir",
            tmp_dir.path().to_str().unwrap(),
        ]);

        let backend = match Backend::new(&matches, &get_null_logger()) {
            Ok(backend) => (backend),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        };

        match backend.get_public_keys() {
            Ok(response) => assert_eq!(response.public_keys.len(), 0),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }

    #[test]
    fn backend_get_public_keys_raw_dir_some_files_are_not_public_keys() {
        let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
        add_sub_dir(&tmp_dir);
        add_key_files(&tmp_dir);
        add_non_key_files(&tmp_dir);

        let matches = set_matches(vec![
            "this_test",
            "--storage-raw-dir",
            tmp_dir.path().to_str().unwrap(),
        ]);

        let backend = match Backend::new(&matches, &get_null_logger()) {
            Ok(backend) => (backend),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        };

        match backend.get_public_keys() {
            Ok(response) => assert_eq!(response.public_keys.len(), 3),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }

    #[test]
    fn backend_get_public_keys_raw_dir_all_files_are_public_keys() {
        let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
        add_key_files(&tmp_dir);

        let matches = set_matches(vec![
            "this_test",
            "--storage-raw-dir",
            tmp_dir.path().to_str().unwrap(),
        ]);

        let backend = match Backend::new(&matches, &get_null_logger()) {
            Ok(backend) => (backend),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        };

        match backend.get_public_keys() {
            Ok(response) => assert_eq!(response.public_keys.len(), 3),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }
}
