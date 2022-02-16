fn main(){
    let s: Vec<String> = {
        let mut line = String::from("Hello paiza");
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    println!("{}", s[0]);
    println!("{}", s[1]);
}
