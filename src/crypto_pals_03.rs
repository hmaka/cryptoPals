use hex;
use std::collections::HashMap;

//const ENGLISH_LETTER_FREQ: std::collections::HashMap<char, f32> = HashMap::new();
const ENGLISH_LETTER_FREQ: &str = "a";
fn single_byte_xor_cypher(cypher_text: &str) -> Vec<String> {
    let mut result_list: Vec<(i32, String)> = Vec::new();

    let mut cypher_text: Vec<u8> = hex::decode(cypher_text).expect("failed to decode");

    for c in ENGLISH_LETTER_FREQ.chars() {
        let cypher_text_xor: Vec<u8> = cypher_text
            .clone()
            .iter()
            .map(|byte| byte ^ c.to_string().as_bytes()[0])
            .collect();
        let cypher_text_xor: String =
            String::from_utf8(cypher_text_xor).expect("failed to convert to str");

        let score = score(cypher_text_xor.as_str());
        result_list.push((score, cypher_text_xor));
    }
    result_list.sort();
    let result_list = result_list.into_iter().map(|tuple| tuple.1).collect();
    return result_list;
}

fn score(cypher_text: &str) -> i32 {
    let mut freq: HashMap<char, i32> = std::collections::HashMap::new();
    let mut score: i32 = 0;

    for c in cypher_text.chars() {
        let c = c.to_ascii_uppercase();
        *freq.entry(c).or_insert(0) += 1;
    }

    for (i, c) in ENGLISH_LETTER_FREQ.chars().enumerate() {
        let f = freq.get(&c).unwrap_or(&0);
        score += (ENGLISH_LETTER_FREQ.len() - i) as i32 * f;
    }
    return score;
}

#[cfg(test)]
#[test]
fn set_1_challenge_3() {
    let cypher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let result_list = single_byte_xor_cypher(cypher_text);

    println!("{:?}", result_list);
}
