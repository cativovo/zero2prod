use std::net::SocketAddr;

use axum::Server;

#[tokio::main]
async fn main() {
    let app = zero2prod::app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
