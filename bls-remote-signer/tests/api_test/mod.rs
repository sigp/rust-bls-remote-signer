use clap::{App, Arg, ArgMatches};
use client::Client;
use environment::{Environment, EnvironmentBuilder};
use std::net::IpAddr::{V4, V6};

pub struct ApiTest {
    pub address: String,
    environment: Environment,
}

impl ApiTest {
    pub fn new() -> Self {
        let matches = get_matches();
        let mut environment = get_environment(false);
        let runtime_context = environment.core_context();

        let client = environment
            .runtime()
            .block_on(Client::new(runtime_context, &matches))
            .map_err(|e| format!("Failed to init Rest API: {}", e))
            .unwrap();

        let address = get_address(&client);

        Self {
            address,
            environment,
        }
    }

    pub fn http_get(url: String) -> serde_json::Value {
        let text = reqwest::blocking::get(&url).unwrap().text().unwrap();
        serde_json::from_str(&text).unwrap()
    }

    pub fn shutdown(mut self) -> () {
        self.environment.fire_signal()
    }
}

fn get_matches() -> ArgMatches<'static> {
    // Declare your CLI parameters.
    // Pro-tip: Use only the ones you need to run this test.
    let matches = App::new("BLS_Remote_Signer")
        .arg(
            Arg::with_name("storage-dummy")
                .long("storage-dummy")
                .help("Dummy flag to check multi storage support")
                .hidden(true)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("storage-raw-dir")
                .long("storage-raw-dir")
                .value_name("DIR")
                .help("Data directory for private keys in raw files."),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("The TCP port to listen on.")
                .default_value("9000")
                .takes_value(true),
        );

    // Now, we will use a custom vec, though
    let arg_vec = vec!["this_test", "--port", "0", "--storage-dummy"];

    // And then, we get our matches
    matches.get_matches_from(arg_vec)
}

fn get_environment(is_log_active: bool) -> Environment {
    let environment_builder = EnvironmentBuilder::new();

    let builder = if is_log_active {
        environment_builder.async_logger("info", None).unwrap()
    } else {
        environment_builder.null_logger().unwrap()
    };

    builder
        .multi_threaded_tokio_runtime()
        .unwrap()
        .build()
        .unwrap()
}

fn get_address(client: &Client) -> String {
    let listening_address = client.get_listening_address();
    let ip = match listening_address.ip() {
        V4(ip) => ip.to_string(),
        V6(ip) => ip.to_string(),
    };

    format!("http://{}:{}", ip, listening_address.port())
}
