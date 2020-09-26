mod constants;

pub use crate::constants::*;
use clap::ArgMatches;
use clap::{App, Arg};
use client::Client;
use environment::{Environment, EnvironmentBuilder};
use std::fs;
use std::fs::{create_dir, File};
use std::io::Write;
use std::net::IpAddr::{V4, V6};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tempdir::TempDir;

pub struct ApiTest {
    pub address: String,
    environment: Environment,
}

impl ApiTest {
    pub fn new(arg_vec: Vec<&str>) -> Self {
        let matches = set_matches(arg_vec);
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

    pub fn shutdown(mut self) {
        self.environment.fire_signal()
    }
}

pub fn set_matches(arg_vec: Vec<&str>) -> ArgMatches<'static> {
    let matches = App::new("BLS_Remote_Signer")
        .arg(
            Arg::with_name("storage-raw-dir")
                .long("storage-raw-dir")
                .value_name("DIR"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .default_value("9000")
                .takes_value(true),
        );

    matches.get_matches_from(arg_vec)
}

pub fn get_environment(is_log_active: bool) -> Environment {
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

pub fn get_address(client: &Client) -> String {
    let listening_address = client.get_listening_address();
    let ip = match listening_address.ip() {
        V4(ip) => ip.to_string(),
        V6(ip) => ip.to_string(),
    };

    format!("http://{}:{}", ip, listening_address.port())
}

pub fn set_permissions(path: &Path, perm_octal: u32) {
    let metadata = fs::metadata(path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(perm_octal);
    fs::set_permissions(path, permissions).unwrap();
}

pub fn add_key_files(tmp_dir: &TempDir) {
    let pairs = vec![
        (PUBLIC_KEY_1, SECRET_KEY_1),
        (PUBLIC_KEY_2, SECRET_KEY_2),
        (PUBLIC_KEY_3, SECRET_KEY_3),
    ];

    add_files(tmp_dir, pairs);
}

pub fn add_mismatched_key_file(tmp_dir: &TempDir) {
    let pairs = vec![(MISMATCHED_PUBLIC_KEY, SECRET_KEY_1)];

    add_files(tmp_dir, pairs);
}

pub fn add_non_key_files(tmp_dir: &TempDir) {
    let pairs = vec![
        (SILLY_FILE_NAME_1, SILLY_CONTENT_1),
        (SILLY_FILE_NAME_2, SILLY_CONTENT_2),
        (SILLY_FILE_NAME_3, SILLY_CONTENT_3),
    ];

    add_files(tmp_dir, pairs);
}

fn add_files(tmp_dir: &TempDir, pairs: Vec<(&str, &str)>) {
    for pair in pairs {
        let file_path = tmp_dir.path().join(pair.0);
        let mut tmp_file = File::create(file_path).unwrap();
        writeln!(tmp_file, "{}", pair.1).unwrap();
    }
}

pub fn add_sub_dirs(tmp_dir: &TempDir) {
    let random_sub_dir_path = tmp_dir.path().join("random_sub_dir_name");
    create_dir(random_sub_dir_path).unwrap();

    let another_sub_dir_path = tmp_dir.path().join(SUB_DIR_NAME);
    create_dir(another_sub_dir_path).unwrap();
}
