mod sign {
    use helpers::*;

    #[test]
    fn additional_field() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let test_block_body = get_test_block_body(0xc137).replace(
            ",\"genesis_validators_root\":\"0x000000000000000000000000000000000000000000000000000000000000c137\"",
            ",\"genesis_validators_root\":\"0x000000000000000000000000000000000000000000000000000000000000c137\", \"foo\":\"bar\"",
        );
        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_ok(response, HAPPY_PATH_BLOCK_SIGNATURE_C137);

        test_signer.shutdown();
    }

    #[test]
    fn storage_error() {
        let (test_signer, tmp_dir) = set_up_api_test_signer_to_sign_message();
        let test_block_body = get_test_block_body(0xc137);
        set_permissions(tmp_dir.path(), 0o40311);
        set_permissions(&tmp_dir.path().join(PUBLIC_KEY_1), 0o40311);

        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let response = http_post_custom_body(&url, &test_block_body);
        set_permissions(tmp_dir.path(), 0o40755);
        set_permissions(&tmp_dir.path().join(PUBLIC_KEY_1), 0o40755);

        assert_sign_error(response, 500, "Storage error: PermissionDenied");

        test_signer.shutdown();
    }

    #[test]
    fn no_public_key_in_path() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let test_block_body = get_test_block_body(0xc137);

        let testcase = |url: String| {
            let response = http_post_custom_body(&url, &test_block_body);
            assert_sign_error(
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
        let test_block_body = get_test_block_body(0xc137);

        let testcase = |url: String| {
            let response = http_post_custom_body(&url, &test_block_body);
            assert_sign_error(
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
        let test_block_body = get_test_block_body(0xc137);

        let testcase = |url: String, expected_err: &str| {
            let response = http_post_custom_body(&url, &test_block_body);
            assert_sign_error(response, 400, expected_err);
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
    fn key_not_found() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, ABSENT_PUBLIC_KEY);
        let test_block_body = get_test_block_body(0xc137);

        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(
            response,
            404,
            &format!("Key not found: {}", ABSENT_PUBLIC_KEY),
        );

        test_signer.shutdown();
    }

    #[test]
    fn invalid_secret_key() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!(
            "{}/sign/{}",
            test_signer.address, PUBLIC_KEY_FOR_INVALID_SECRET_KEY
        );
        let test_block_body = get_test_block_body(0xc137);

        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(
            response,
            500,
            &format!(
                "Invalid secret key: public_key: {}; Invalid hex character: W at index 0",
                PUBLIC_KEY_FOR_INVALID_SECRET_KEY
            ),
        );

        test_signer.shutdown();
    }

    #[test]
    fn key_mismatch() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, MISMATCHED_PUBLIC_KEY);
        let test_block_body = get_test_block_body(0xc137);

        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(
            response,
            500,
            &format!("Key mismatch: {}", MISMATCHED_PUBLIC_KEY),
        );

        test_signer.shutdown();
    }

    #[test]
    fn invalid_json() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |custom_body: &str, expected_err: &str| {
            let response = http_post_custom_body(&url, custom_body);
            assert_sign_error(response, 400, expected_err);
        };

        testcase(
            "Trolololololo",
            "Unable to parse body message from JSON: Error(\"expected value\", line: 1, column: 1)",
        );
        testcase(
            "{\"bls_domain\"}",
            "Unable to parse body message from JSON: Error(\"expected `:`\", line: 1, column: 14)",
        );
        testcase(
            "{\"bls_domain\":}",
            "Unable to parse body message from JSON: Error(\"expected value\", line: 1, column: 15)",
        );

        testcase(
            "{\"bls_domain\":\"}",
            "Unable to parse body message from JSON: Error(\"EOF while parsing a string\", line: 1, column: 16)",
        );

        test_signer.shutdown();
    }

    #[test]
    fn invalid_field_bls_domain() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |json_patch, expected_err| {
            let test_block_body = get_test_block_body(0xc137).replace(
                "\"bls_domain\":\"beacon_proposer\"",
                &format!("\"bls_domain\":{}", json_patch),
            );
            let response = http_post_custom_body(&url, &test_block_body);
            assert_sign_error(response, 400, expected_err);
        };

        testcase("\"blah\"", "Unsupported bls_domain parameter: blah");
        testcase("\"domain\"", "Unsupported bls_domain parameter: domain");
        testcase("\"\"", "Unsupported bls_domain parameter: ");
        testcase("", "Unable to parse body message from JSON: Error(\"expected value\", line: 1, column: 15)");
        testcase("1", "Unable to parse body message from JSON: Error(\"invalid type: integer `1`, expected a string\", line: 1, column: 15)");
        testcase("true", "Unable to parse body message from JSON: Error(\"invalid type: boolean `true`, expected a string\", line: 1, column: 18)");
        testcase("{\"cats\":\"3\"}", "Unable to parse body message from JSON: Error(\"invalid type: map, expected a string\", line: 1, column: 14)");
        testcase("[\"a\"]", "Unable to parse body message from JSON: Error(\"invalid type: sequence, expected a string\", line: 1, column: 14)");

        test_signer.shutdown();
    }

    #[test]
    fn missing_field_bls_domain() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let test_block_body =
            get_test_block_body(0xc137).replace("\"bls_domain\":\"beacon_proposer\",", "");
        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(response, 400, "Unable to parse body message from JSON: Error(\"missing field `bls_domain`\", line: 1, column: 236311)");

        test_signer.shutdown();
    }

    #[test]
    fn invalid_field_fork() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |json_patch, expected_err| {
            let test_block_body = get_test_block_body(0xc137).replace(
                "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",\"epoch\":1545},",
                json_patch,
            );

            let response = http_post_custom_body(&url, &test_block_body);
            assert_sign_error(response, 400, expected_err);
        };

        testcase(
            "\"fork\":{\"current_version\":\"0x02020202\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"missing field `previous_version`\", line: 1, column: 236214)",
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"missing field `current_version`\", line: 1, column: 236215)",
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",",
            "Unable to parse body message from JSON: Error(\"missing field `epoch`\", line: 1, column: 236328)",
        );
        testcase(
            "\"fork\":{\"previous_version\":\"INVALID_VALUE\",\"current_version\":\"0x02020202\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"missing 0x prefix\", line: 1, column: 236204)",
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0xINVALID_VALUE\",\"current_version\":\"0x02020202\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"invalid hex (OddLength)\", line: 1, column: 236206)",
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0xINVALID_VALUE_\",\"current_version\":\"0x02020202\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"invalid hex (InvalidHexCharacter { c: \\\'I\\\', index: 0 })\", line: 1, column: 236207)",
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"INVALID_VALUE\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"missing 0x prefix\", line: 1, column: 236235)"
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0xINVALID_VALUE\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"invalid hex (OddLength)\", line: 1, column: 236237)"
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0xINVALID_VALUE_\",\"epoch\":1545},",
            "Unable to parse body message from JSON: Error(\"invalid hex (InvalidHexCharacter { c: \\\'I\\\', index: 0 })\", line: 1, column: 236238)"
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",\"epoch\":},",
            "Unable to parse body message from JSON: Error(\"expected value\", line: 1, column: 236242)"
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",\"epoch\":\"zzz\"},",
            "Unable to parse body message from JSON: Error(\"invalid type: string \\\"zzz\\\", expected u64\", line: 1, column: 236246)"
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",\"epoch\":true},",
            "Unable to parse body message from JSON: Error(\"invalid type: boolean `true`, expected u64\", line: 1, column: 236245)"
        );
        testcase(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",\"epoch\":[\"a\"]},",
            "Unable to parse body message from JSON: Error(\"invalid type: sequence, expected u64\", line: 1, column: 236241)"
        );

        test_signer.shutdown();
    }

    #[test]
    fn missing_field_fork() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let test_block_body = get_test_block_body(0xc137).replace(
            "\"fork\":{\"previous_version\":\"0x01010101\",\"current_version\":\"0x02020202\",\"epoch\":1545},",
            "",
        );
        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(response, 400, "Unable to parse body message from JSON: Error(\"missing field `fork`\", line: 1, column: 236257)");

        test_signer.shutdown();
    }

    #[test]
    fn missing_field_data() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let test_block_body = get_test_block_body(0xc137).replace("\"data\":", "\"not-data\":");

        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(response, 400, "Unable to parse body message from JSON: Error(\"missing field `data`\", line: 1, column: 236938)");

        test_signer.shutdown();
    }

    #[test]
    fn invalid_field_genesis_validators_root() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let testcase = |json_patch, expected_err| {
            let test_block_body = get_test_block_body(0xc137).replace(
                ",\"genesis_validators_root\":\"0x000000000000000000000000000000000000000000000000000000000000c137\"",
                &format!(",\"genesis_validators_root\":{}", json_patch),
            );

            let response = http_post_custom_body(&url, &test_block_body);
            assert_sign_error(response, 400, expected_err);
        };

        testcase("\"0\"", "Unable to parse body message from JSON: Error(\"0x prefix is missing\", line: 1, column: 236276)");
        testcase("\"0x\"", "Unable to parse body message from JSON: Error(\"invalid length 0, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236277)");
        testcase("\"0xa\"", "Unable to parse body message from JSON: Error(\"invalid length 1, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236278)");
        testcase("\"deadbeef\"", "Unable to parse body message from JSON: Error(\"0x prefix is missing\", line: 1, column: 236283)");
        testcase("\"0xdeadbeefzz\"", "Unable to parse body message from JSON: Error(\"invalid length 10, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236287)");
        testcase("\"0xdeadbeef1\"", "Unable to parse body message from JSON: Error(\"invalid length 9, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236286)");
        testcase("", "Unable to parse body message from JSON: Error(\"expected value\", line: 1, column: 236274)");
        testcase("1", "Unable to parse body message from JSON: Error(\"invalid type: integer `1`, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236274)");
        testcase("true", "Unable to parse body message from JSON: Error(\"invalid type: boolean `true`, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236277)");
        testcase("{\"cats\":\"3\"}", "Unable to parse body message from JSON: Error(\"invalid type: map, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236273)");
        testcase("[\"a\"]", "Unable to parse body message from JSON: Error(\"invalid type: sequence, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236273)");
        testcase(
            "\"0x000000000000000000000000000000000000000000000000000000000000c1370\"",
            "Unable to parse body message from JSON: Error(\"invalid length 65, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236342)",
        );
        testcase(
            "\"0x000000000000000000000000000000000000000000000000000000000000c13700\"",
            "Unable to parse body message from JSON: Error(\"invalid length 66, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236343)",
        );
        testcase(
            "\"0x000000000000000000000000000000000000000000000000000000000000c1370000\"",
            "Unable to parse body message from JSON: Error(\"invalid length 68, expected a 0x-prefixed hex string with length of 64\", line: 1, column: 236345)",
        );

        test_signer.shutdown();
    }

    #[test]
    fn missing_field_genesis_validators_root() {
        let (test_signer, _tmp_dir) = set_up_api_test_signer_to_sign_message();
        let url = format!("{}/sign/{}", test_signer.address, PUBLIC_KEY_1);

        let test_block_body = get_test_block_body(0xc137).replace(
            ",\"genesis_validators_root\":\"0x000000000000000000000000000000000000000000000000000000000000c137\"",
            "",
        );
        let response = http_post_custom_body(&url, &test_block_body);
        assert_sign_error(response, 400, "Unable to parse body message from JSON: Error(\"missing field `genesis_validators_root`\", line: 1, column: 236247)");

        test_signer.shutdown();
    }
}
