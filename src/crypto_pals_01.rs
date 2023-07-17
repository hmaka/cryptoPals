use base64;
use hex;

pub fn hex_to_base64(s: String) -> String {
    let hex_in_bytes: Vec<u8> = hex::decode(s).expect("Failed to convert hex string to raw bytes");
    let base64_string = base64::encode(&hex_in_bytes);
    return base64_string;
}

#[cfg(test)]
#[test]
fn set_1_challenge_1() {
    use std::assert_eq;

    use crate::crypto_pals_01;

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();

    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();

    let converted = crypto_pals_01::hex_to_base64(input);
    assert_eq!(converted, expected);
}
