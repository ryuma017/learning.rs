use std::io;

fn main() {
    let win_nums = {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };

    let n = {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<i64>().unwrap()
    };

    for _ in 0..n {
        let guess_nums = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            line
        };
        let mut result = 0;

        for num in guess_nums.split_whitespace() {
            if win_nums.iter().any(|e| e == num) {
                result += 1;
            }
        }
        println!("{}", result);
    }
}
