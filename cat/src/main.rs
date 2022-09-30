fn run(path: String) {
    match std::fs::read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    let mut args = std::env::args();
    match args.nth(1) {
        Some(path) => run(path),
        None => println!("Usage: cat <path>"),
    }
}
