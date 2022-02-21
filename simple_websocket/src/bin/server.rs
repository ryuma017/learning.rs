use ws::listen;

fn main() {
    println!("running server...\n");

    listen("127.0.0.1:8000", |out| {
        move |msg| {
            let response: String = format!("{}", msg);

            println!("{}", response);

            out.send(response)
        }
    })
    .unwrap()
}
