fn convert_fahrenheit_into_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn main() {
    let mut i = String::new();
    std::io::stdin().read_line(&mut i).ok();

    let i: i32 = i.trim().parse().ok().unwrap();

    let converted_i = convert_fahrenheit_into_celsius(i);

    println!("{}", converted_i);
}
