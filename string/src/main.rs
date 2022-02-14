#![allow(unused)]

fn main() {
    /// 新しい空のストリングを生成
    let mut s = String::new();

    /// to_stringメソッドを使用して文字列リテラルからStringを生成
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    /// String::from関数を使って文字列リテラルからStringを生成
    let s = String::from("initial contents");

    /// push_strメソッドでStringに文字列スライスを追加する
    let mut s = String::from("foo");
    s.push_str("bar");

    /// 中身をStringに追加した後に、文字列スライスを使用する
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s2の所有権は奪われない
    println!("s2 is {}", s2); // 問題なく動く

    /// + 演算子を使用して2つのString値を新しいString値にする
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、使用できない

    /// format!マクロを使用した文字列の連結
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // 引数の所有権は奪われない

    /// 文字列をスライスする
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    /// 文字列を走査するメソッド
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
