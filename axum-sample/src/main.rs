use axum_sample::startup::ApiServer;

#[tokio::main]
async fn main() {
    let server = ApiServer::build();
    server.run().await;
}
