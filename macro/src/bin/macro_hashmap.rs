macro_rules! hashmap {
    ( $($key:expr => $val:expr),* ) => {{
        // HashMapを生成
        let mut tmp = std::collections::HashMap::new();
        $(
            tmp.insert($key, $val);
        )*
        tmp
    }}
}

fn main() {
    // マクロを利用してHashMapを初期化
    let week = hashmap![
        "mon" => "月曜",
        "tue" => "火曜",
        "wed" => "水曜",
        "thu" => "木曜",
        "fri" => "金曜",
        "sat" => "土曜",
        "sun" => "日曜"
    ];
    println!("mon={}", week["mon"]);
    println!("wed={}", week["wed"]);
}
