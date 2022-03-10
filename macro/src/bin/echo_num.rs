// [書式] 宣言的マクロ(declarative macro)の定義
// macro_rules! マクロ名 {
//     (構文規則1) => { 生成するプログラム1 };
//     (構文規則2) => { 生成するプログラム2 };
//     ...
// }

// 数値を画面に表示するマクロを定義
macro_rules! echo_num {
    ($num:expr) => {
        println!("{}", $num);
    };
}

fn main() {
    echo_num!(10);
    echo_num![20];
    echo_num! {30}
}
