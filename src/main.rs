pub mod crypto_pals_01;
pub mod crypto_pals_02;
pub mod crypto_pals_03;
pub mod crypto_pals_04;
pub mod crypto_pals_05;
pub mod crypto_pals_06;
pub mod helpers;

fn main() {
    let file_name = "Frankenstein.txt";
    helpers::get_letter_freq(file_name);
}
