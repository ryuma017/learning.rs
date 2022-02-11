#![allow(unused, clippy::vec_init_then_push)]

fn main() {
    // 新しい空のベクタを生成してi32型の値を保持する
    let v: Vec<i32> = Vec::new();

    // 値を含む新しいベクタを生成する
    let v = vec![1, 2, 3];

    // pushメソッドを使用してベクタ型に値を追加する
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // ベクタとその要素がドロップする箇所を示す
    {
        let v = vec![1, 2, 3, 4];

        // vに関する処理

    } // <- vはここでスコープを抜け、解放される

    // 添字記法かgetメソッドを使用してベクタの要素にアクセスする
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // forループで要素を走査し、ベクタの各要素を出力する
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // ベクタ要素への可変な参照を走査する
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
    println!("{:?}", v); // -> [150, 82, 107]

    // enumを定義して、一つのベクタに異なる型の値を保持する
    // (ベクタは同一の型しか保持できない)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
