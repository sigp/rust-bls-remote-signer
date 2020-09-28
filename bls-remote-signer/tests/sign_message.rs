mod sign_message {
    use client::api_response::SignatureApiResponse;
    use helpers::*;
    use std::collections::HashMap;

    fn post(url: &str, signing_root: &str) -> ApiTestResponse {
        let mut hashmap = HashMap::new();
        hashmap.insert("signingRoot", signing_root);
        hashmap.insert("other_field", "The signer should ignore this field.");

        http_post(url, hashmap)
    }

    fn assert_ok(resp: ApiTestResponse, expected_signature: &str) {
        assert_eq!(resp.status, 200);
        assert_eq!(
            serde_json::from_value::<SignatureApiResponse>(resp.json)
                .unwrap()
                .signature,
            expected_signature
        );
    }

    fn assert_error(resp: ApiTestResponse, http_status: u16, error_msg: &str) {
        assert_eq!(resp.status, http_status);
        assert_eq!(resp.json["error"], error_msg);
    }

    #[test]
    fn storage_error() {
        let (test_signer, tmp_dir) = set_up_api_test_signer_to_sign_message();
        set_permissions(tmp_dir.path(), 0o40311);
        set_permissions(&tmp_dir.path().join(PUBLIC_KEY_1), 0o40311);

        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let response = post(&url, SIGNING_ROOT);
        set_permissions(tmp_dir.path(), 0o40755);
        set_permissions(&tmp_dir.path().join(PUBLIC_KEY_1), 0o40755);

        assert_error(response, 500, "Storage error: PermissionDenied");

        test_signer.shutdown();
    }

    #[test]
    fn no_public_key_in_path() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let testcase = |url: String| {
            let response = post(&url, SIGNING_ROOT);

            assert_error(
                response,
                400,
                "Parameter public_key needed in route /sign/:public_key",
            );
        };

        testcase(format!("{}/sign/", test_signer.address));
        testcase(format!("{}/sign", test_signer.address));
        testcase(format!("{}/sign//", test_signer.address));
        testcase(format!("{}/sign///", test_signer.address));
        testcase(format!("{}/sign/?'or 1 = 1 --", test_signer.address));

        test_signer.shutdown();
    }

    #[test]
    fn additional_path_segments() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let testcase = |url: String| {
            let response = post(&url, SIGNING_ROOT);

            assert_error(
                response,
                400,
                "Only one path segment is allowed after /sign",
            );
        };

        testcase(format!("{}/sign/this/receipt", test_signer.address));
        testcase(format!("{}/sign/this/receipt/please", test_signer.address));
        testcase(format!("{}/sign/this/receipt/please?", test_signer.address));
        testcase(format!(
            "{}/sign//{}/valid/pk",
            test_signer.address, PUBLIC_KEY_1
        ));

        test_signer.shutdown();
    }

    #[test]
    fn invalid_public_key() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let testcase = |url: String, expected_err: &str| {
            let response = post(&url, SIGNING_ROOT);

            assert_error(response, 400, expected_err);
        };

        testcase(
            format!("{}/sign/{}", test_signer.address, "ScottBakula"),
            "Invalid public key: ScottBakula",
        );
        testcase(
            format!("{}/sign/{}", test_signer.address, "deadbeef"),
            "Invalid public key: deadbeef",
        );
        testcase(
            format!("{}/sign/{}", test_signer.address, SILLY_FILE_NAME_1),
            &format!("Invalid public key: {}", SILLY_FILE_NAME_1),
        );
        testcase(
            format!("{}/sign/{}", test_signer.address, SILLY_FILE_NAME_1),
            &format!("Invalid public key: {}", SILLY_FILE_NAME_1),
        );
        testcase(
            format!("{}/sign/0x{}", test_signer.address, PUBLIC_KEY_1),
            &format!("Invalid public key: 0x{}", PUBLIC_KEY_1),
        );
        testcase(
            format!("{}/sign/{}55", test_signer.address, PUBLIC_KEY_1),
            &format!("Invalid public key: {}55", PUBLIC_KEY_1),
        );

        test_signer.shutdown();
    }

    #[test]
    fn invalid_json() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |custom_body: &str, expected_err: &str| {
            let response = http_post_custom_body(&url, custom_body);

            assert_error(response, 400, expected_err);
        };

        testcase(
            "Trolololololo",
            "Unable to parse JSON: Error(\"expected value\", line: 1, column: 1)",
        );
        testcase(
            "{\"signingRoot\"}",
            "Unable to parse JSON: Error(\"expected `:`\", line: 1, column: 15)",
        );
        testcase(
            "{\"signingRoot\":}",
            "Unable to parse JSON: Error(\"expected value\", line: 1, column: 16)",
        );

        testcase(
            "{\"signingRoot\":\"}",
            "Unable to parse JSON: Error(\"EOF while parsing a string\", line: 1, column: 17)",
        );

        test_signer.shutdown();
    }

    #[test]
    fn missing_signing_root_in_json() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |custom_body: &str, expected_err: &str| {
            let response = http_post_custom_body(&url, custom_body);

            assert_error(response, 400, expected_err);
        };

        testcase("{\"signingRoot\":\"\"}", "Empty field signingRoot.");

        test_signer.shutdown();
    }

    #[test]
    fn signing_root_in_json_not_a_string() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |custom_body: &str, expected_err: &str| {
            let response = http_post_custom_body(&url, custom_body);

            assert_error(response, 400, expected_err);
        };

        testcase("{\"signingRoot\":1}", "Invalid field signingRoot: 1");
        testcase("{\"signingRoot\":true}", "Invalid field signingRoot: true");
        testcase(
            "{\"signingRoot\":{\"cats\":\"3\"}}",
            "Invalid field signingRoot: {\"cats\":\"3\"}",
        );
        testcase(
            "{\"signingRoot\":[\"a\"]}",
            "Invalid field signingRoot: [\"a\"]",
        );

        test_signer.shutdown();
    }

    #[test]
    fn invalid_string_signing_root() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |signing_root: &str, expected_err: &str| {
            let response = post(&url, signing_root);

            assert_error(response, 400, expected_err);
        };

        testcase("0", "Invalid signingRoot: 0");
        testcase("0x", "Invalid signingRoot: 0x");
        testcase("0xa", "Invalid signingRoot: 0xa");
        testcase("deadbeef", "Invalid signingRoot: deadbeef");
        testcase("0xdeadbeefzz", "Invalid signingRoot: 0xdeadbeefzz");
        testcase("0xdeadbeef1", "Invalid signingRoot: 0xdeadbeef1");

        test_signer.shutdown();
    }

    #[test]
    fn key_not_found() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let url = format!("{}/sign/{}", test_signer.address, ABSENT_PUBLIC_KEY);

        let response = post(&url, SIGNING_ROOT);

        assert_error(
            response,
            404,
            &format!("Key not found: {}", ABSENT_PUBLIC_KEY),
        );

        test_signer.shutdown();
    }

    #[test]
    fn key_mismatch() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let url = format!("{}/sign/{}", test_signer.address, MISMATCHED_PUBLIC_KEY);

        let response = post(&url, SIGNING_ROOT);

        assert_error(
            response,
            500,
            &format!("Key mismatch: {}", MISMATCHED_PUBLIC_KEY),
        );

        test_signer.shutdown();
    }

    #[test]
    fn happy_path() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();

        let testcase = |pk: &str, exp_sign: &str| {
            let url = format!("{}/sign/{}", test_signer.address, pk);

            let response = post(&url, SIGNING_ROOT);
            assert_ok(response, exp_sign);
        };

        testcase(PUBLIC_KEY_1, EXPECTED_SIGNATURE_1);
        testcase(PUBLIC_KEY_2, EXPECTED_SIGNATURE_2);
        testcase(PUBLIC_KEY_3, EXPECTED_SIGNATURE_3);

        test_signer.shutdown();
    }
}
