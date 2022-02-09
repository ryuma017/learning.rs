fn main(){
    let s: Vec<String> = {
        let mut line = String::from("one two three four five");
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    for c in s {
        println!("{}", c);
    }
}
