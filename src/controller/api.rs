use serde::{Serialize, Deserialize};
use axum::Json;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub count: i32
}

pub async fn api_handler() -> Json<Response> {
    let response = Response {
        count: 3
    };
    Json(response)
}

pub async fn post_api_handler() -> Json<Response> {
    let response = Response {
        count: 4
    };
    Json(response)
}
