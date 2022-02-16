#![allow(unused)]

use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // ファイルを開く
    let f = File::open("hello.txt");

    // match式を使用して返却される可能性のあるResult列挙子を処理する
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        }
    };
}
