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
mod tests {
    use super::*;
    use helpers::*;

    #[test]
    fn backend_utils_hex_string_to_bytes() {
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
            vec![
                104, 8, 26, 254, 183, 173, 62, 141, 70, 159, 135, 1, 8, 4, 195, 232, 213, 62, 247,
                125, 57, 48, 89, 165, 81, 50, 99, 114, 6, 204, 89, 236
            ]
        );

        assert_eq!(
            hex_string_to_bytes(&PUBLIC_KEY_1).unwrap(),
            vec![
                183, 53, 66, 82, 170, 91, 206, 39, 171, 149, 55, 253, 1, 88, 81, 89, 53, 243, 195,
                134, 20, 25, 225, 180, 182, 200, 33, 155, 93, 189, 21, 252, 249, 7, 189, 223, 39,
                84, 66, 243, 227, 47, 144, 79, 121, 128, 122, 42
            ]
        );

        assert_eq!(
            hex_string_to_bytes(&SIGNING_ROOT[2..]).unwrap(),
            vec![
                182, 187, 143, 55, 101, 249, 63, 79, 30, 124, 115, 72, 71, 146, 137, 201, 38, 19,
                153, 163, 198, 144, 102, 133, 227, 32, 7, 26, 26, 19, 149, 92
            ]
        );

        assert_eq!(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_1[2..]).unwrap(),
            vec![
                181, 208, 192, 28, 239, 59, 2, 142, 44, 95, 53, 124, 45, 75, 136, 111, 142, 55, 77,
                9, 221, 102, 12, 215, 221, 20, 104, 13, 79, 149, 103, 120, 128, 139, 77, 59, 42,
                183, 67, 232, 144, 252, 26, 119, 174, 98, 195, 201, 13, 97, 53, 97, 178, 60, 106,
                218, 235, 91, 14, 40, 136, 50, 48, 79, 221, 192, 140, 116, 21, 8, 11, 231, 62, 85,
                110, 136, 98, 161, 180, 208, 246, 170, 128, 132, 227, 74, 144, 21, 68, 213, 187,
                106, 238, 211, 166, 18
            ]
        );

        assert_eq!(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_2[2..]).unwrap(),
            vec![
                182, 182, 62, 60, 236, 208, 150, 125, 159, 155, 144, 227, 238, 17, 61, 251, 33,
                236, 211, 144, 29, 188, 101, 76, 166, 150, 73, 172, 90, 7, 70, 117, 134, 97, 48,
                102, 39, 241, 139, 182, 215, 166, 234, 3, 172, 224, 105, 80, 14, 231, 154, 40, 21,
                76, 23, 45, 215, 31, 254, 75, 113, 24, 117, 228, 139, 96, 70, 106, 144, 243, 164,
                220, 172, 219, 201, 181, 245, 67, 74, 214, 140, 145, 230, 3, 254, 23, 3, 50, 77,
                131, 97, 127, 82, 112, 174, 173
            ]
        );

        assert_eq!(
            hex_string_to_bytes(&EXPECTED_SIGNATURE_3[2..]).unwrap(),
            vec![
                135, 79, 125, 109, 65, 116, 223, 16, 136, 171, 64, 189, 154, 60, 128, 133, 84, 197,
                93, 109, 225, 223, 252, 172, 199, 239, 86, 195, 202, 34, 226, 11, 82, 162, 61, 213,
                187, 101, 104, 161, 35, 181, 157, 240, 186, 206, 243, 222, 20, 212, 193, 151, 162,
                251, 42, 88, 104, 161, 140, 75, 17, 246, 215, 149, 118, 115, 217, 163, 2, 191, 104,
                18, 177, 213, 223, 155, 38, 69, 4, 246, 130, 180, 61, 251, 207, 79, 145, 48, 203,
                94, 187, 155, 142, 55, 55, 222
            ]
        );

        assert_eq!(
            hex_string_to_bytes(&"0a0b11".to_string()).unwrap(),
            vec![10, 11, 17]
        );
    }

    #[test]
    fn backend_utils_get_signature_string() {
        let signature_1: [u8; 96] = [
            181, 208, 192, 28, 239, 59, 2, 142, 44, 95, 53, 124, 45, 75, 136, 111, 142, 55, 77, 9,
            221, 102, 12, 215, 221, 20, 104, 13, 79, 149, 103, 120, 128, 139, 77, 59, 42, 183, 67,
            232, 144, 252, 26, 119, 174, 98, 195, 201, 13, 97, 53, 97, 178, 60, 106, 218, 235, 91,
            14, 40, 136, 50, 48, 79, 221, 192, 140, 116, 21, 8, 11, 231, 62, 85, 110, 136, 98, 161,
            180, 208, 246, 170, 128, 132, 227, 74, 144, 21, 68, 213, 187, 106, 238, 211, 166, 18,
        ];

        assert_eq!(
            bytes96_to_hex_string(signature_1).unwrap(),
            EXPECTED_SIGNATURE_1
        );

        let signature_2: [u8; 96] = [
            182, 182, 62, 60, 236, 208, 150, 125, 159, 155, 144, 227, 238, 17, 61, 251, 33, 236,
            211, 144, 29, 188, 101, 76, 166, 150, 73, 172, 90, 7, 70, 117, 134, 97, 48, 102, 39,
            241, 139, 182, 215, 166, 234, 3, 172, 224, 105, 80, 14, 231, 154, 40, 21, 76, 23, 45,
            215, 31, 254, 75, 113, 24, 117, 228, 139, 96, 70, 106, 144, 243, 164, 220, 172, 219,
            201, 181, 245, 67, 74, 214, 140, 145, 230, 3, 254, 23, 3, 50, 77, 131, 97, 127, 82,
            112, 174, 173,
        ];

        assert_eq!(
            bytes96_to_hex_string(signature_2).unwrap(),
            EXPECTED_SIGNATURE_2
        );

        let signature_3: [u8; 96] = [
            135, 79, 125, 109, 65, 116, 223, 16, 136, 171, 64, 189, 154, 60, 128, 133, 84, 197, 93,
            109, 225, 223, 252, 172, 199, 239, 86, 195, 202, 34, 226, 11, 82, 162, 61, 213, 187,
            101, 104, 161, 35, 181, 157, 240, 186, 206, 243, 222, 20, 212, 193, 151, 162, 251, 42,
            88, 104, 161, 140, 75, 17, 246, 215, 149, 118, 115, 217, 163, 2, 191, 104, 18, 177,
            213, 223, 155, 38, 69, 4, 246, 130, 180, 61, 251, 207, 79, 145, 48, 203, 94, 187, 155,
            142, 55, 55, 222,
        ];

        assert_eq!(
            bytes96_to_hex_string(signature_3).unwrap(),
            EXPECTED_SIGNATURE_3
        );
    }
}
