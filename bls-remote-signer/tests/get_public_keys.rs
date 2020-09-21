use api_test::ApiTest;
use serde::Deserialize;

pub mod api_test;

#[derive(Deserialize)]
struct PublicKeysResponse {
    public_keys: Vec<String>,
}

#[test]
fn get_public_keys() {
    // TODO
    // For the real test (not the one with storagedummy),
    // You need to create a tmp dir, and give it some priv keys file.

    let api_test = ApiTest::new();

    let url = format!("{}/publicKeys", api_test.address);
    let resp = ApiTest::http_get(url);
    let resp: PublicKeysResponse = serde_json::from_value(resp).unwrap();

    // TODO
    // Let's just test for now the number of keys returned.public_keys.
    // We want to test the following:
    // - Number of returned keys.
    // - That each key is a public key.
    // - No additional fields than `public_keys` in the response.
    // - Appropiate Header in the response.
    assert_eq!(resp.public_keys.len(), 6);

    api_test.shutdown();
}
