use client::api_error::ApiErrorDesc;
use client_backend::PublicKeys;
use helpers::*;
use tempdir::TempDir;

#[test]
fn integration_get_public_keys_all_files_in_dir_are_public_keys() {
    let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
    add_key_files(&tmp_dir);

    let arg_vec = vec![
        "this_test",
        "--port",
        "0",
        "--storage-raw-dir",
        tmp_dir.path().to_str().unwrap(),
    ];
    let api_test = ApiTest::new(arg_vec);

    let url = format!("{}/publicKeys", api_test.address);
    let resp = ApiTest::http_get(url);
    let resp: PublicKeys = serde_json::from_value(resp).unwrap();

    assert_eq!(resp.public_keys.len(), 3);

    api_test.shutdown();
}

#[test]
fn integration_get_public_keys_some_files_in_dir_are_public_keys() {
    let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
    add_sub_dirs(&tmp_dir);
    add_key_files(&tmp_dir);
    add_non_key_files(&tmp_dir);

    let arg_vec = vec![
        "this_test",
        "--port",
        "0",
        "--storage-raw-dir",
        tmp_dir.path().to_str().unwrap(),
    ];
    let api_test = ApiTest::new(arg_vec);

    let url = format!("{}/publicKeys", api_test.address);
    let resp = ApiTest::http_get(url);
    let resp: PublicKeys = serde_json::from_value(resp).unwrap();

    assert_eq!(resp.public_keys.len(), 3);

    api_test.shutdown();
}

#[test]
fn integration_get_public_keys_no_files_in_dir_are_public_keys() {
    let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
    add_sub_dirs(&tmp_dir);
    add_non_key_files(&tmp_dir);

    let arg_vec = vec![
        "this_test",
        "--port",
        "0",
        "--storage-raw-dir",
        tmp_dir.path().to_str().unwrap(),
    ];
    let api_test = ApiTest::new(arg_vec);

    let url = format!("{}/publicKeys", api_test.address);
    let resp = ApiTest::http_get(url);
    let resp: PublicKeys = serde_json::from_value(resp).unwrap();

    // TODO
    // We want to 404 on no keys
    assert_eq!(resp.public_keys.len(), 0);

    api_test.shutdown();
}

#[test]
fn integration_get_public_keys_directory_failure() {
    let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
    add_sub_dirs(&tmp_dir);
    add_key_files(&tmp_dir);
    add_non_key_files(&tmp_dir);

    let arg_vec = vec![
        "this_test",
        "--port",
        "0",
        "--storage-raw-dir",
        tmp_dir.path().to_str().unwrap(),
    ];
    let api_test = ApiTest::new(arg_vec);

    // Somebody tripped over a wire.
    set_permissions(tmp_dir.path(), 0o40311);

    let url = format!("{}/publicKeys", api_test.address);
    let resp = ApiTest::http_get(url);

    let resp: ApiErrorDesc = serde_json::from_value(resp).unwrap();

    // Be able to delete the tempdir afterward, regardless of this test result.
    set_permissions(tmp_dir.path(), 0o40755);

    assert_eq!(resp.error, String::from("Storage error: PermissionDenied."));

    api_test.shutdown();
}
