use std::fmt::{Error, Write};

// While `hex::decode` provides this functionality, we are implemeting
// here to be able to work on "zeroization" if required.
pub fn hex_string_to_bytes(data: &str) -> Result<Vec<u8>, String> {
    // TODO
    // See note on "zeroization" at this crate's `lib.rs`.
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

#[cfg(test)]
mod utils {
    use super::*;
    use helpers::*;

    #[test]
    fn fn_hex_string_to_bytes() {
        let compare = |v1: Vec<u8>, v2: Vec<u8>| v1.iter().zip(v2.iter()).all(|(a, b)| a == b);

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

        assert!(compare(
            hex_string_to_bytes(&PUBLIC_KEY_1).unwrap(),
            PUBLIC_KEY_1_BYTES.to_vec()
        ));

        assert!(compare(
            hex_string_to_bytes(&SIGNING_ROOT[2..]).unwrap(),
            SIGNING_ROOT_BYTES.to_vec()
        ));

        assert!(compare(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_1[2..]).unwrap(),
            EXPECTED_SIGNATURE_1_BYTES.to_vec()
        ));

        assert!(compare(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_2[2..]).unwrap(),
            EXPECTED_SIGNATURE_2_BYTES.to_vec()
        ));

        assert!(compare(
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
}
