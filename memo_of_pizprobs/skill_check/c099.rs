fn main() {
    let (n, d): (i32, i32) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut width = d;

    for _ in 0..n - 1 {
        let x: i32 = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<i32>().unwrap()
        };

        width += d - x;
    }

    println! {"{}", width*d};
}
