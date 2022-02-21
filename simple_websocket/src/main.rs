use ws::listen;

fn main() {
    println!("Running Server");

    listen("127.0.0.1:8000", |out| {
        move |msg| {
            let response: String = format!("{} - Im server", msg);

            println!("{}", response);

            out.send(response)
        }
    }).unwrap()
}
