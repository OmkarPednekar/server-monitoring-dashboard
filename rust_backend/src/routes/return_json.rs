use axum::Json;
use serde_json::{ json, Value };
pub async fn return_some_json() -> Json<Value> {
    let json = json!({"message":"Hello World"});
    Json(json)
}
