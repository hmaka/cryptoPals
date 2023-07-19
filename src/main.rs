pub mod create_letter_freq;
pub mod crypto_pals_01;
pub mod crypto_pals_02;
pub mod crypto_pals_03;

fn main() {
    let file_name = "Frankenstein.txt";
    create_letter_freq::get_letter_freq(file_name);
}
