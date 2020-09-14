pub mod api_error;
mod config;
mod rest_api;
mod router;

use clap::ArgMatches;
use config::Config;
use environment::RuntimeContext;
use std::net::Ipv4Addr;

pub struct Client {}

impl Client {
    pub async fn new(context: RuntimeContext, cli_args: &ArgMatches<'_>) -> Result<Self, String> {
        let mut config = Config::default();

        if let Some(address) = cli_args.value_of("listen-address") {
            config.listen_address = address
                .parse::<Ipv4Addr>()
                .map_err(|_| "listen-address is not a valid IPv4 address.")?;
        }

        if let Some(port) = cli_args.value_of("port") {
            config.port = port
                .parse::<u16>()
                .map_err(|_| "port is not a valid u16.")?;
        }

        // TODO
        // Setup backend

        let _listening_address = rest_api::start_server(context.executor, config)
            .map_err(|e| format!("Failed to start HTTP API: {:?}", e))?;

        Ok(Self {})
    }
}
