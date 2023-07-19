use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn get_letter_freq(file_name: &str) -> io::Result<()> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    let mut letter_count: i32 = 0;

    let mut freq: HashMap<char, i32> = HashMap::new();
    for line in reader.lines() {
        let line = line?;

        for c in line.chars() {
            if c.is_ascii_lowercase() {
                letter_count += 1;
                freq.entry(c)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }
    let mut expected_freq: HashMap<char, f32> = HashMap::new();
    for (c, count) in freq.into_iter() {
        expected_freq.insert(c, count as f32 / letter_count as f32);
    }

    write_to_file(expected_freq);
    return Ok(());
}

pub fn write_to_file(freq_data: HashMap<char, f32>) {
    let freq_data_json = serde_json::to_string_pretty(&freq_data).unwrap();

    let mut file = File::create("letter_freq_data.txt").unwrap();
    file.write_all(freq_data_json.as_bytes()).unwrap();
}
