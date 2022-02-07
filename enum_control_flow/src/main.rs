#![allow(unused)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[allow(clippy::manual_map)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let v = value_in_cents(Coin::Quarter(UsState::Alaska));
    print!("{}", v);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // ------------------------------ //

    let coin = Coin::Penny;
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State Quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // 上の let if記法 と同義
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
}
