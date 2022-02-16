fn main() {
    let (s1, s2, s3): (String, String, String) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}
