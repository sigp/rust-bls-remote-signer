pub mod api_error;
mod config;
mod rest_api;
mod router;

use config::Config;
use environment::RuntimeContext;

pub struct Client {}

impl Client {
    pub async fn new(context: RuntimeContext) -> Result<Self, String> {
        // TODO
        // Parse config
        let config = Config::default();

        // TODO
        // Setup backend

        let _listening_address = rest_api::start_server(context.executor, config)
            .map_err(|e| format!("Failed to start HTTP API: {:?}", e))?;

        Ok(Self {})
    }
}
