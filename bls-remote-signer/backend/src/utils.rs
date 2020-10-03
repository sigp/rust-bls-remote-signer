use crate::BackendError;
use bls::SecretKey;
use std::fmt::{Error, Write};

// While `hex::decode` provides this functionality, we are implemeting
// here to be able to work on "zeroization" if required.
pub fn hex_string_to_bytes(data: &str) -> Result<Vec<u8>, String> {
    if data.len() % 2 != 0 {
        return Err("Odd length".to_string());
    }

    data.as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(i, pair)| Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?))
        .collect()
}

fn val(c: u8, idx: usize) -> Result<u8, String> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(format!(
            "Invalid hex character: {} at index {}",
            c as char, idx
        )),
    }
}

// hex::encode only allows up to 32 bytes.
pub fn bytes96_to_hex_string(data: [u8; 96]) -> Result<String, Error> {
    static CHARS: &[u8] = b"0123456789abcdef";
    let mut s = String::with_capacity(96 * 2 + 2);

    s.write_char('0')?;
    s.write_char('x')?;

    for &byte in data.iter() {
        s.write_char(CHARS[(byte >> 4) as usize].into())?;
        s.write_char(CHARS[(byte & 0xf) as usize].into())?;
    }

    Ok(s)
}

/// Computes the public key from the retrieved `secret_key` and compares it
/// with the given `public_key` parameter, returning a deserialized SecretKey.
pub fn validate_bls_pair(public_key: &str, secret_key: &str) -> Result<SecretKey, BackendError> {
    let deserialize = |sk: &str| -> Result<SecretKey, String> {
        let sk = hex_string_to_bytes(&sk)?;
        Ok(SecretKey::deserialize(&sk).map_err(|e| format!("{:?}", e))?)
    };

    let secret_key: SecretKey = deserialize(secret_key).map_err(|e| {
        BackendError::InvalidSecretKey(format!("public_key: {}; {}", public_key, e))
    })?;

    let pk_param_as_bytes = hex_string_to_bytes(&public_key)
        .map_err(|e| BackendError::InvalidPublicKey(format!("{}; {}", public_key, e)))?;

    if &secret_key.public_key().serialize()[..] != pk_param_as_bytes {
        return Err(BackendError::KeyMismatch(public_key.to_string()));
    }

    Ok(secret_key)
}

#[cfg(test)]
mod utils {
    use super::*;
    use helpers::*;

    fn compare_vec_u8(v1: Vec<u8>, v2: Vec<u8>) -> bool {
        v1.iter().zip(v2.iter()).all(|(a, b)| a == b)
    }

    #[test]
    fn fn_hex_string_to_bytes() {
        assert_eq!(
            hex_string_to_bytes(&"0aa".to_string()).err(),
            Some("Odd length".to_string())
        );

        assert_eq!(
            hex_string_to_bytes(&"0xdeadbeef".to_string()).err(),
            Some("Invalid hex character: x at index 1".to_string())
        );

        assert_eq!(
            hex_string_to_bytes(&"n00bn00b".to_string()).err(),
            Some("Invalid hex character: n at index 0".to_string())
        );

        assert_eq!(
            hex_string_to_bytes(&"abcdefgh".to_string()).err(),
            Some("Invalid hex character: g at index 6".to_string())
        );

        assert_eq!(
            hex_string_to_bytes(&SECRET_KEY_1).unwrap(),
            SECRET_KEY_1_BYTES
        );

        assert!(compare_vec_u8(
            hex_string_to_bytes(&PUBLIC_KEY_1).unwrap(),
            PUBLIC_KEY_1_BYTES.to_vec()
        ));

        assert!(compare_vec_u8(
            hex_string_to_bytes(&SIGNING_ROOT[2..]).unwrap(),
            SIGNING_ROOT_BYTES.to_vec()
        ));

        assert!(compare_vec_u8(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_1[2..]).unwrap(),
            EXPECTED_SIGNATURE_1_BYTES.to_vec()
        ));

        assert!(compare_vec_u8(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_2[2..]).unwrap(),
            EXPECTED_SIGNATURE_2_BYTES.to_vec()
        ));

        assert!(compare_vec_u8(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_3[2..]).unwrap(),
            EXPECTED_SIGNATURE_3_BYTES.to_vec()
        ));

        assert_eq!(
            hex_string_to_bytes(&"0a0b11".to_string()).unwrap(),
            vec![10, 11, 17]
        );
    }

    #[test]
    fn fn_bytes96_to_hex_string() {
        assert_eq!(
            bytes96_to_hex_string(EXPECTED_SIGNATURE_1_BYTES).unwrap(),
            EXPECTED_SIGNATURE_1
        );

        assert_eq!(
            bytes96_to_hex_string(EXPECTED_SIGNATURE_2_BYTES).unwrap(),
            EXPECTED_SIGNATURE_2
        );

        assert_eq!(
            bytes96_to_hex_string(EXPECTED_SIGNATURE_3_BYTES).unwrap(),
            EXPECTED_SIGNATURE_3
        );
    }

    #[test]
    fn fn_validate_bls_pair() {
        let test_ok_case = |pk: &str, sk: &str, sk_bytes: &[u8; 32]| {
            let serialized_secret_key = validate_bls_pair(pk, sk).unwrap().serialize();
            assert!(compare_vec_u8(
                serialized_secret_key.as_bytes().to_vec(),
                sk_bytes.to_vec()
            ));
        };

        test_ok_case(PUBLIC_KEY_1, SECRET_KEY_1, &SECRET_KEY_1_BYTES);

        let test_error_case = |pk: &str, sk: &str, expected_error: &str| {
            assert_eq!(
                validate_bls_pair(pk, sk).err().unwrap().to_string(),
                expected_error
            );
        };

        test_error_case(
            PUBLIC_KEY_2,
            &"TamperedKey%#$#%#$$&##00£$%$$£%$".to_string(),
            &format!(
                "Invalid secret key: public_key: {}; Invalid hex character: T at index 0",
                PUBLIC_KEY_2
            ),
        );

        test_error_case(
            PUBLIC_KEY_2,
            &"deadbeef".to_string(),
            &format!(
                "Invalid secret key: public_key: {}; InvalidSecretKeyLength {{ got: 4, expected: 32 }}",
                PUBLIC_KEY_2
            ),
        );

        let bad_pk_param = "not_validated_by_the_api_handler!";
        test_error_case(
            bad_pk_param,
            SECRET_KEY_1,
            &format!("Invalid public key: {}; Odd length", bad_pk_param),
        );

        test_error_case(
            PUBLIC_KEY_1,
            SECRET_KEY_2,
            &format!("Key mismatch: {}", PUBLIC_KEY_1),
        );

        test_error_case(
            PUBLIC_KEY_2,
            SECRET_KEY_3,
            &format!("Key mismatch: {}", PUBLIC_KEY_2),
        );
    }
}
