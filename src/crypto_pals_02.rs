use hex;

fn xor_two_equal_length_buffers(hex_string1: String, hex_string2: String) -> String {
    let hex1_in_bytes: Vec<u8> = hex::decode(hex_string1).expect("Failed to decode");
    let hex2_in_bytes: Vec<u8> = hex::decode(hex_string2).expect("Failed to decode");
    //let a = String::from_utf8(hex1_in_bytes.clone()).expect("didn't work");
    //let b = String::from_utf8(hex2_in_bytes.clone()).expect("didn't work");
    //println!("test{}{}", a, b);

    let xor_result: Vec<u8> = hex1_in_bytes
        .into_iter()
        .zip(hex2_in_bytes.into_iter())
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect();
    let xor_result: String = hex::encode(xor_result);

    return xor_result;
}

#[cfg(test)]
#[test]
fn set_1_challenge_2() {
    use std::assert_eq;

    let input1 = "1c0111001f010100061a024b53535009181c".to_string();
    let input2 = "686974207468652062756c6c277320657965".to_string();

    let expected_result = "746865206b696420646f6e277420706c6179".to_string();
    let actual_result = xor_two_equal_length_buffers(input1, input2);
    assert_eq!(expected_result, actual_result);
}
