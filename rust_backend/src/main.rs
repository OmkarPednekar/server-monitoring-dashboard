// use serde::{ Serialize, Deserialize };
use std::net::SocketAddr;
use tokio::net::TcpListener;
use rust_backend::routes;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, routes::routes()).await.unwrap();
}
