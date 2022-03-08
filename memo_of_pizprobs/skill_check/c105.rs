fn main() {
    let n: usize = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    let mut card_list: Vec<u32> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect()
    };

    card_list.sort();

    let mut total = card_list[n - 1];

    for i in 0..n - 1 {
        if card_list[i + 1] - card_list[i] != 1 {
            total += card_list[i];
        }
    }

    println!("{}", total);
}
