pub fn hamming_distance_strings(str1: &str, str2: &str) -> i32 {
    let str1_as_bytes: Vec<u8> = str1.as_bytes().to_vec();
    let str2_as_bytes: Vec<u8> = str2.as_bytes().to_vec();

    let mut hamming_distance: i32 =
        (str1_as_bytes.len() as i32 - str2_as_bytes.len() as i32).abs() * 4;

    for (byte1, byte2) in str1_as_bytes.iter().zip(str2_as_bytes.iter()) {
        let byte1_xor_byte2 = byte1 ^ byte2;
        hamming_distance += byte1_xor_byte2.count_ones() as i32;
    }

    return hamming_distance;
}

#[cfg(test)]
#[test]
fn hamming_distance_test() {
    use std::assert_eq;

    let str1 = "this is a test";
    let str2 = "wokka wokka!!!";

    assert_eq!(hamming_distance_strings(str1, str2), 37);
}
