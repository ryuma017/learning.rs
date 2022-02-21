use ws::{connect, CloseCode};

fn main() {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).unwrap();

    connect("ws://127.0.0.1:8000", |out| {
        out.send(text.as_str()).unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    })
    .unwrap()
}
