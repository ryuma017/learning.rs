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
}