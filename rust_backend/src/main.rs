use axum::{ Router, routing::get, Json, routing::post };
use serde_json::{ json, value, Value };
use serde::{ Serialize, Deserialize };
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn return_some_json() -> Json<Value> {
    let json = json!({"hello":"world"});
    Json(json)
}
#[derive(Deserialize)]
struct data {
    num_1: u64,
    num_2: u64,
}
#[derive(Serialize)]
struct res {
    sum: u64,
}

async fn sum(Json(body): Json<data>) -> Json<res> {
    let total: u64 = body.num_1 + body.num_2;
    return Json(res { sum: total });
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(return_some_json)).route("/sum", post(sum));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();
}
