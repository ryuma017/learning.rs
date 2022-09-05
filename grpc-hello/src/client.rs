use tokio::io::{AsyncBufReadExt, BufReader};

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50052").await?;
    let mut reader = BufReader::new(tokio::io::stdin());
    loop {
        let mut msg = String::new();
        let bytes_len = reader.read_line(&mut msg).await.unwrap();
        if bytes_len == 0 {
            break;
        }
        let request = tonic::Request::new(HelloRequest {
            name: msg.trim().into(),
        });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response);
    }
    Ok(())
}
