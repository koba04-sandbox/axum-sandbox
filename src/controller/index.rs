use axum::response::Html;

pub async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <meta charset="utf-8">
            </head>
            <body>
                <h1>Hello Axum</h1>
                <p id="count">Loading...</p>
                <button id="increment">++</button>
                <script>
                    const btn = document.getElementById("increment");
                    const text = document.getElementById("count");
                    fetch("/api").then(res => res.json()).then(res => {
                        text.textContent = res.count;
                    });
                    document.getElementById("increment").addEventListener("click", () => {
                        fetch("/api", { method: "POST" }).then(res => res.json()).then(res => {
                            text.textContent = res.count;
                        })
                    });
                </script>
            </body>
        </html>
        "#
    )
}