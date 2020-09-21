use crate::{BackendError, Storage};

#[derive(Copy, Clone)]
pub struct StorageRawDir {}

impl StorageRawDir {
    pub fn new() -> Result<Self, String> {
        // TODO
        // - take the dir param
        // - verify you have access to that dir
        // - add the dir to your struct
        // - return the object, Ok

        Ok(Self {})
    }
}

impl Storage for StorageRawDir {
    fn get_public_keys(self: Box<Self>) -> Result<Vec<String>, BackendError> {
        todo!()
    }

    fn get_private_key(self: Box<Self>, _input: &str) -> Result<Vec<u8>, BackendError> {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn Storage> {
        Box::new(*self)
    }
}
