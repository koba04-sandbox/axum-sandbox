use std::env;

use axum::{
    routing::{get, get_service, post},
    Router,
};
use tower_http::services::ServeDir;
use toy_app::{api_handler, post_api_handler, root_handler};

#[tokio::main]
async fn main() {
    let router = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn app() -> Router {
    let redis_url = env::var("REDIS_URL").unwrap();
    let redis = redis::Client::open(redis_url).unwrap();
    let serve_static = get_service(ServeDir::new("public"));

    Router::new()
        .route("/", get(root_handler))
        .route("/api", get(api_handler))
        .route("/api", post(post_api_handler))
        .nest_service("/public", serve_static)
        .with_state(redis)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use toy_app::Response;

    #[tokio::test]
    async fn returns_index_html() {
        let router = app().await;
        let address = "127.0.0.1:8000";
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let html = reqwest::get(format!("http://{}/", address))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        assert!(html.contains("<h1>Hello Axum</h1>"));
    }

    #[tokio::test]
    async fn update_count_through_api() {
        let router = app().await;
        let address = "127.0.0.1:8001";
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let client = reqwest::Client::new();
        let json = client
            .get(format!("http://{}/api", address))
            .send()
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();
        let count = json.count;

        let client = reqwest::Client::new();
        let json = client
            .post(format!("http://{}/api", address))
            .send()
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();

        assert_eq!(json.count, count + 1);
    }
}
