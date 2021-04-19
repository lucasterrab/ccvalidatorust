extern crate regex;

use regex::Regex;
use std::io;

fn validate_credit_card(cc: &str) -> (bool, String) {
    (luhn_check(cc), get_card_type(cc))
}

fn luhn_check(cc: &str) -> bool {
    let cc_digits: Vec<u32> = cc.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
    let cc_len = cc_digits.len();
 
    let checksum = cc_digits[..cc_len - 1]
        .iter()
        .enumerate()
        .fold(0, |s, (n, d)| {
            s + if (cc_len-n % 2) % 2 == 0 { sum_digits(2 * d) } else { *d }
        });

    checksum % 10 == 0
}
 
fn sum_digits(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn get_card_type(cc: &str) -> String {
    let visa = Regex::new("^4[0-9]{12}(?:[0-9]{3})?$");
    let master_card = Regex::new("^5[1-5][0-9]{14}$");

    if visa.unwrap().is_match(cc) {
        String::from("Visa")
    } else if master_card.unwrap().is_match(cc) {
        String::from("MasterCard")
    } else {
        String::from("Unknown card type")
    }
}

fn main() {
    let mut cc = String::new();

    println!("Please, enter the credit card number to be validated: ");
    io::stdin()
        .read_line(&mut cc)
        .expect("Failed to read the credit card number.");

    let l = cc.len();
    
    println!("{} {:?}", cc, validate_credit_card(&cc[..l - 1]));
}