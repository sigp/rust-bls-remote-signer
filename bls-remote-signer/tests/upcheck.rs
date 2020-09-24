use helpers::ApiTest;
use tempdir::TempDir;

#[test]
fn integration_get_upcheck() {
    let tmp_dir = TempDir::new("bls-remote-signer-test").unwrap();
    let arg_vec = vec![
        "this_test",
        "--port",
        "0",
        "--storage-raw-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let api_test = ApiTest::new(arg_vec);

    let url = format!("{}/upcheck", api_test.address);
    let resp = ApiTest::http_get(url);

    assert_eq!(resp["status"], "OK");

    api_test.shutdown();
}
