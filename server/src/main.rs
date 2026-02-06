use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/test", get(test));

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    
    println!("Listening at http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn test() -> &'static str {
    "Hello World!"
}
