use axum::{routing::{get,post}, Router};
use toy_app::{root_handler, api_handler, post_api_handler};

#[tokio::main]
async fn main() {
    let router = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn app() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/api", get(api_handler))
        .route("/api", post(post_api_handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use toy_app::Response;

    #[tokio::test]
    async fn it_works() {
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
        assert_eq!(html, "<h1>Hello</h1>");
    }

    #[tokio::test]
    async fn api_returns_json() {
        let router = app().await;
        let address = "127.0.0.1:8001";
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let json = reqwest::get(format!("http://{}/api", address))
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();
        assert_eq!(json.count, 3);
    }
}
