use crate::{BackendError, Storage};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_dir;
use std::path::PathBuf;

lazy_static! {
    static ref PUBLIC_KEY_REGEX: Regex = Regex::new(r"[0-9a-fA-F]{96}").unwrap();
}

#[derive(Clone)]
pub struct StorageRawDir {
    path: PathBuf,
}

impl StorageRawDir {
    /// Initialized the storage with the given path, verifying
    /// whether if it is a directory and if its available to the user.
    /// Does not list, nor verify the contents of the directory.
    pub fn new(path: &str) -> Result<Self, String> {
        let path = PathBuf::from(path);

        if !path.exists() {
            return Err("Path does not exist.".to_string());
        }

        if !path.is_dir() {
            return Err("Path is not a directory.".to_string());
        }

        match read_dir(path.clone()) {
            Ok(_) => Ok(Self { path }),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl Storage for StorageRawDir {
    /// List all the files in the directory having a BLS public key name.
    /// This function DOES NOT check the contents of each file.
    /// The latter functionality is performed by `self.get_private_key()`.
    fn get_public_keys(self: Box<Self>) -> Result<Vec<String>, BackendError> {
        let entries = read_dir(self.path).map_err(|e| BackendError::StorageError(e.to_string()));

        // We are silently suppressing errors in this chain
        // because we only care about files actually passing these filters.
        let public_keys: Vec<String> = entries? // unwrap. We checked for error above
            .filter_map(|entry| entry.ok()) // consume the Ok(s)
            .filter(|entry| !entry.path().is_dir()) // only take the files, not the dirs
            .map(|entry| entry.file_name().into_string()) // OsString to Result<String, OsString>
            .filter_map(|entry| entry.ok()) // consume the Ok(s)
            .filter(|name| PUBLIC_KEY_REGEX.is_match(name)) // only take the public key names
            .collect();

        Ok(public_keys)
    }

    fn get_private_key(self: Box<Self>, _input: &str) -> Result<Vec<u8>, BackendError> {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn Storage> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::new_storage_with_tmp_dir;
    use helpers::*;

    #[test]
    fn storage_raw_dir_problem_with_path() {
        let (storage, tmp_dir) = new_storage_with_tmp_dir();
        add_key_files(&tmp_dir);

        // All good and fancy, let's make the dir innacessible now.
        set_permissions(tmp_dir.path(), 0o40311);

        // TODO
        // Pay attention to this error message --> Permission denied (os error 13)
        // We may want to switch to a regular expression strategy.
        match storage.get_public_keys() {
            Ok(_) => panic!("This invocation to Backend::new() should return error"),
            Err(e) => assert_eq!(e.to_string(), "Storage: Permission denied (os error 13)",),
        }

        // Give permissions back, we want the tempdir to be deleted.
        set_permissions(tmp_dir.path(), 0o40755);
    }

    #[test]
    fn storage_raw_dir_no_files_in_dir() {
        let (storage, _tmp_dir) = new_storage_with_tmp_dir();

        match storage.get_public_keys() {
            Ok(keys) => assert_eq!(keys.len(), 0),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }

    #[test]
    fn storage_raw_dir_there_are_files_in_dir_none_are_keys() {
        let (storage, tmp_dir) = new_storage_with_tmp_dir();
        add_sub_dir(&tmp_dir); // To spice things up.
        add_non_key_files(&tmp_dir);

        match storage.get_public_keys() {
            Ok(keys) => assert_eq!(keys.len(), 0),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }

    #[test]
    fn storage_raw_dir_not_all_files_have_public_key_names() {
        let (storage, tmp_dir) = new_storage_with_tmp_dir();
        add_sub_dir(&tmp_dir);
        add_key_files(&tmp_dir);
        add_non_key_files(&tmp_dir);

        match storage.get_public_keys() {
            Ok(keys) => assert_eq!(keys.len(), 3),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }

    #[test]
    fn storage_raw_dir_all_files_do_have_public_key_names() {
        let (storage, tmp_dir) = new_storage_with_tmp_dir();
        add_key_files(&tmp_dir);

        match storage.get_public_keys() {
            Ok(keys) => assert_eq!(keys.len(), 3),
            Err(e) => panic!("We should not be getting an err here: {}", e),
        }
    }
}
