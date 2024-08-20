use axum::{ Router, routing::get };
use http::Method;
mod return_json;
use return_json::return_some_json;
use tower_http::cors::{ Any, CorsLayer };
pub fn routes() -> Router {
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    Router::new().route("/", get(return_some_json)).layer(cors)
}
