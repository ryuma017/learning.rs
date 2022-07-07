#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;

use error_stack::{IntoReport, Report, Result, ResultExt};

#[derive(Debug)]
struct ParsePaymentInfoError;

impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

impl Error for ParsePaymentInfoError {}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {}

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

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card was found for {name}");
        Report::new(CreditCardError::InvalidInput(msg.clone()))
            .attach_printable(msg)
    })?;

    let card = parse_card(card_string)
        .change_context(CreditCardError::Other)
        .attach_printable(format!("{name}'s card could not be parsed."))?;

    Ok(card)
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let expected_len = 4;
    let len = numbers.len();

    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError))
            .attach_printable(format!(
                "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
            ))
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
            number,
            exp: Expiration { year, month },
            cvv,
        })
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(' ')
        .into_iter()
        .map(|s| {
            s.parse()
            .report()
            .attach_printable_lazy(|| {
                format!("{s:?} could not be parsed as u32")
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePaymentInfoError)
        .attach_printable(format!(
            "Failed to parse input as number. Input: {card}"
        ))?;

    Ok(numbers)
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
