fn main() {
    let n: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    let mut ball_l: Vec<i64> = (0..n)
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim().parse().unwrap()
        })
        .collect();

    let m: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    for _ in 0..m {
        let (a, b, num): (usize, usize, i64) = {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        };

        if num <= ball_l[a - 1] {
            ball_l[a - 1] -= num;
            ball_l[b - 1] += num;
        } else {
            ball_l[b - 1] += ball_l[a - 1];
            ball_l[a - 1] = 0;
        }
    }

    println!(
        "{}",
        ball_l
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
