use serde::{Serialize, Deserialize};
use axum::{Json, extract::State};
use redis::Commands;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub count: i32
}

pub async fn api_handler(State(mut client): State<redis::Client>) -> Json<Response> {
    let count: i32 = client.get("count").unwrap();
    let response = Response {
        count
    };
    Json(response)
}

pub async fn post_api_handler(State(mut client): State<redis::Client>) -> Json<Response> {
    let mut count: i32 = client.get("count").unwrap();
    count += 1;
    let _: () = client.set("count", count).unwrap();
    let response = Response {
        count
    };
    Json(response)
}
