#![allow(unused)]

use std::{ collections::HashMap, str::SplitAsciiWhitespace};

fn main() {
    /// ハッシュマップを生成してキーと値を挿入する
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /// 2つのリストからハッシュマップを生成する
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /// 一旦挿入されたら、キーと値はハッシュマップに所有されることを示す
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_nameとfield_valueはこの時点で無効になる。

    /// ハッシュマップに保持されたキーの値にアクセスする
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    /// forループでハッシュマップのキーと値のペアを走査する
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /// 特定のキーで保持された値を置き換える
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    /// entryメソッドを使ってキーに値がない場合だけ挿入する
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    /// 単語とカウントを保持するハッシュマップを使って単語の出現数をカウントする
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    /// 練習
    /// 1. 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(中央値)、mode(最頻値)を返す
    let mut numbers = vec![1, 3, 1, 2, 3, 1, 3, 3, 2];

    let mean = {
        let sum: f64 = numbers.iter().map(|&val| val as f64).sum();
        let length: f64 = numbers.len() as f64;

        sum / length
    };

    #[allow(clippy::stable_sort_primitive)]
    let median = {
        numbers.sort(); // try: `v.sort_unstable()`ってclippyが言ってた。等しい要素の順序が保証されないけど高速らしい。
        let mid = numbers.len() / 2;
        if numbers.len() % 2 == 0 {
            (numbers[mid-1] + numbers[mid]) / 2
        } else {
            numbers[mid]
        }
    };

    let mode: Vec<i32> = {
        let mut map = HashMap::new();
        for n in &numbers {
            let count = map.entry(n).or_insert(0);
            *count += 1;
        }

        let max_value = map.values().cloned().max().unwrap();

        map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
    };

    println!("{:?}", numbers);
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode[0]);
}
