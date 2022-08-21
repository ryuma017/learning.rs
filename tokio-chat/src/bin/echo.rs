use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            loop {
                let mut msg = String::new();
                let bytes_len = reader.read_line(&mut msg).await.unwrap();
                if bytes_len == 0 {
                    break;
                }
                writer.write_all(msg.as_bytes()).await.unwrap();
            }
        });
    }
}
