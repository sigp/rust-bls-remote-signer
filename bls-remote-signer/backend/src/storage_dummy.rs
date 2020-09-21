use crate::{BackendError, Storage};

// TODO
// We will move this implementation to the tests.

#[derive(Copy, Clone)]
pub struct StorageDummy {}

impl StorageDummy {
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }
}

impl Storage for StorageDummy {
    fn get_public_keys(self: Box<Self>) -> Result<Vec<String>, BackendError> {
        Ok(vec!(
            "b7697ace759f71549dec92059b74373ef48832543bbf777f2a08a97c80d44f13dd92d9eb27397961bad21e6bde52c7b5".to_string(),
            "ac6fa6ea0258909a4847dc16ca10e1d3de3df0cfa8329f98fe95438825917d67254f2d5747e7816467b4d381d80f31ac".to_string(),
            "8e57f806fa387394b5efc94594e259b09e3e4953d3a6a58f074ed17bcea923f37b22e15964781cc7ba12969a17850465".to_string(),
            "84143cbe58e4c13e13d218759550de68f0238902c450f0c5dfbe0e9206b6891610f9bd558dbbed686e73ff028f62c141".to_string(),
            "a9f3cf5e8d1ca7737c77356f61102aff3fe321935cc032bc94ca48e744565b341db2b3cda11c6d25f4e6c312fd13356f".to_string(),
            "8bf7ed92df260150deefb519cb66d6d9989ddded814306e71b1868b1822c54faa033010a93f199519ecd3dcd167c455f".to_string(),
        ))
    }

    fn get_private_key(self: Box<Self>, _input: &str) -> Result<Vec<u8>, BackendError> {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn Storage> {
        Box::new(*self)
    }
}
