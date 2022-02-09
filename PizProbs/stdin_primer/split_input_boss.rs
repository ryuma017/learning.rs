fn main() {
    let s: Vec<String> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    for c in s {
        println!("{}", c);
    }
}
