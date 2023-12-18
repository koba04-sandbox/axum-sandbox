use askama::Template;
use axum::{extract::State, response::Html};
use redis::Commands;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    count: i32,
}

pub async fn root_handler(State(mut client): State<redis::Client>) -> Html<String> {
    let count: i32 = client.get("count").unwrap();
    let template = IndexTemplate { count };
    Html(template.render().unwrap())
}
