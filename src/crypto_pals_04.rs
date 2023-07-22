use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::crypto_pals_03;

fn find_xor_encrypted_line_in_file(file_name: &str) -> Vec<(f32, String)> {
    let file = File::open(file_name).unwrap();

    let mut result_list: Vec<(f32, String)> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_as_sting = line.unwrap();
        let xor_decryptions_for_line =
            crypto_pals_03::single_byte_xor_cypher(line_as_sting.as_str());

        if xor_decryptions_for_line.len() > 0 {
            result_list.push(xor_decryptions_for_line[0].clone());
        }
    }
    result_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return result_list;
}

#[cfg(test)]
#[test]

fn set_1_challenge_4() {
    use std::assert_eq;

    let result = find_xor_encrypted_line_in_file("crypopals_static_challenge-data_4.txt");
    let expected = "Now that the party is jumping\n";

    // as before, used the code to find the plain text then hard coded it as a test.
    // println!("{:?}", result[0]);
    assert_eq!(expected, result[0].1)
}
