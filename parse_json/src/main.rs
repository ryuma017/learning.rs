use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u8,
    have_gf: bool
}

fn main() -> Result<()> {
    let path = Path::new("sample.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let person_vec: Vec<Person> = serde_json::from_reader(reader)?;

    for person in person_vec {
        println!(
            "name: {}, age: {}, have_girlfrend: {}.",
            person.name, person.age, person.have_gf
        );
    }

    Ok(())
}