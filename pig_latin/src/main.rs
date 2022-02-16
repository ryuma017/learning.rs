fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let words = input.trim().split_whitespace();
    let result = words.map(pig_latin).collect::<Vec<_>>().join(" ");

    println!("{}", result);
}

fn pig_latin(word: &str) -> String {
    let mut letters = word.chars();
    let first_letter = letters.next().unwrap();

    let lowercase_first_letter = first_letter.to_lowercase().next().unwrap();

    match lowercase_first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", letters.as_str(), lowercase_first_letter),
    }
}