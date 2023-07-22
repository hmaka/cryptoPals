pub fn repeated_key_xor_encryption(plain_text: &str, key: &str) -> Vec<u8> {
    // encryption buffer, the plain text as bytes on which we will perform our repeated xor
    let mut encryption_buffer: Vec<u8> = plain_text.as_bytes().to_vec();
    let key_as_bytes: Vec<u8> = key.as_bytes().to_vec();

    //let mut encryption_buffer: Vec<u8> = hex::decode(plain_text).expect("failed to decode");
    //let key_as_bytes: Vec<u8> = hex::decode(key).expect("failed to decode");

    for (i, byte) in encryption_buffer.iter_mut().enumerate() {
        *byte ^= key_as_bytes[i % key_as_bytes.len()];
    }
    return encryption_buffer;
}

#[cfg(test)]
#[test]
fn set_1_challenge_5() {
    use std::assert_eq;

    let plain_text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let result = repeated_key_xor_encryption(plain_text, key);
    let expected = hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").expect("cannot decode");

    assert_eq!(result, expected);
}
