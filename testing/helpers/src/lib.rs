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
    // Declare your CLI parameters.
    // Pro-tip: Use only the ones you need to run this test.
    let matches = App::new("BLS_Remote_Signer")
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

    // And apply your matches
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
    let pairs = vec!(
            ("68081afeb7ad3e8d469f87010804c3e8d53ef77d393059a55132637206cc59ec",
            "b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a",
            ),
            ("45b5e876e5e57b23af3e86c37d708626cf1dcca6a650091bba2ddb3e0b7304ae",
            "9324739760579527b4f8c34c5df42f9fd89f59fdbe8a01d58675769f60fec5da9b9c8d7a3203cf2217692e49e7b98d97"
            ),
            ("1e52a4e54e89ccba813d5f902545749c356f6187341b4e765bf43ece401762f6",
            "8244ac66a8bffa0ce0af04d69ed7ed009951061259173a7c7ae1f25c049f0fcbbf2fad67b6d2b276a697315be755dac5"
            ),
        );

    for pair in &pairs {
        let file_path = tmp_dir.path().join(pair.1);
        let mut tmp_file = File::create(file_path).unwrap();
        writeln!(tmp_file, "{}", pair.0).unwrap();
    }
}

pub fn add_non_key_files(tmp_dir: &TempDir) {
    let pairs = vec!(
            ("HemanandtheMastersoftheUniverse",
            "IAmAdamPrinceofEterniaDefenderofthesecretsoftheCAstleGrayskullThisisCringermyfearlessfriend",
            ),
            ("Centurions",
            "InthenearfutureDocTerrorandhiscyborgcompanianHackerunleashtheirforcestoconquerEarth"
            ),
            ("CaptainPlanet",
            "OurworldisinperilGaiathespiritoftheearthcannolongerstandtheterribledestructionplaguingourplanet"
            ),
        );

    for pair in &pairs {
        let file_path = tmp_dir.path().join(pair.1);
        let mut tmp_file = File::create(file_path).unwrap();
        writeln!(tmp_file, "{}", pair.0).unwrap();
    }
}

pub fn add_sub_dir(tmp_dir: &TempDir) {
    let random_sub_dir_path = tmp_dir.path().join("random_sub_dir");
    create_dir(random_sub_dir_path).unwrap();
}
