use hex;
use std::collections::HashMap;

const ENGLISH_LETTER_FREQ: &str = include_str!("../letter_freq_data.txt");
pub fn single_byte_xor_cypher(cypher_text: &str) -> Vec<(f32, String)> {
    let mut result_list: Vec<(f32, String)> = Vec::new();

    let cypher_text: Vec<u8> = hex::decode(cypher_text).expect("failed to decode");
    let all_bytes: Vec<u8> = (0..=255).collect();

    for b in all_bytes {
        let cypher_text_xor: Vec<u8> = cypher_text.clone().iter().map(|byte| byte ^ b).collect();

        match String::from_utf8(cypher_text_xor) {
            Ok(cypher_text_xor_as_string) => {
                let score = score(&cypher_text_xor_as_string);
                result_list.push((score, cypher_text_xor_as_string));
            }
            _ => {}
        }
    }
    result_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //let result_list = result_list.into_iter().map(|tuple| tuple.1).collect();
    return result_list;
}

fn score(cypher_text: &str) -> f32 {
    let english_letter_freq: HashMap<char, f32> =
        serde_json::from_str(ENGLISH_LETTER_FREQ).unwrap();
    let mut freq: HashMap<char, i32> = std::collections::HashMap::new();
    let mut score: f32 = 0.0;

    for c in cypher_text.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    for (c, n) in english_letter_freq.iter() {
        let f: f32 = *freq.get(&c).unwrap_or(&0) as f32 / cypher_text.len() as f32;
        let freq_distance = f - n;
        score += freq_distance.abs();
    }
    return score;
}

#[cfg(test)]
#[test]
fn set_1_challenge_3() {
    use std::assert_eq;

    let cypher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let result_list = single_byte_xor_cypher(cypher_text);

    // println!("{:?}", result_list[0]);
    // I found the plain text using this code then after the fact created this test with plain text
    // hard coded.
    assert_eq!(result_list[0].1, "Cooking MC's like a pound of bacon");
}
