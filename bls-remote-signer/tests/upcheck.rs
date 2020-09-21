use api_test::ApiTest;

// Workaround (adding `pub` to get rid of a nagging unused code warning,
// when not using a helper in one file, even when we are using it in other.
// https://github.com/rust-lang/rust/issues/46379#issuecomment-548787629
pub mod api_test;

#[test]
fn upcheck() {
    let api_test = ApiTest::new();

    let url = format!("{}/upcheck", api_test.address);
    let resp = ApiTest::http_get(url);

    assert_eq!(resp["status"], "OK");

    api_test.shutdown();
}
