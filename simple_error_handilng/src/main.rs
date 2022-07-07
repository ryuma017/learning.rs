#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;

use error_stack::{IntoReport, Report, Result, ResultExt};

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Card {
    let card_string = credit_cards.get(name).unwrap();

    parse_card(card_string)
}

fn parse_card(card: &str) -> Card {
    let mut numbers = parse_card_numbers(card);

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Card {
        number,
        exp: Expiration { year, month },
        cvv,
    }
}

fn parse_card_numbers(card: &str) -> Vec<u32> {
    let numbers = card
        .split(' ')
        // .into_iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    numbers
}

fn main() {
    let credit_cards = HashMap::from([
        ("Ryuma", "1234567 07 25 123"),
        ("Rio", "1234567 10 22 123"),
        ("Fuma", "1234567 07 25 123"),
    ]);

    println!("Enter Name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let result = get_credit_card_info(&credit_cards, name.trim());

    println!("\nCredit Card Info: {result:?}");
}
