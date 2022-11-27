use std::net::SocketAddr;

use axum::{
    body::Body,
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;

use crate::handlers::index;

pub struct ApiServer {
    _port: u16,
    server: Server<AddrIncoming, IntoMakeService<Router<Body>>>,
}

impl ApiServer {
    pub fn build() -> Self {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

        let server = axum::Server::bind(&addr)
            .serve(Router::new().route("/", get(index)).into_make_service());

        Self {
            _port: addr.port(),
            server,
        }
    }

    pub async fn run(self) {
        self.server.await.unwrap();
    }
}
