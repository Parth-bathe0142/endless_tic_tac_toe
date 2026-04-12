use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};



#[tokio::main]
async fn main() {
    let pool = sqlx::SqlitePool::connect("sqlite:./database.db?mode=rwc").await.unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("../frontend/build")
            .not_found_service(ServeFile::new("../frontend/build/index.html")))
        .route("/test", get(test))
        .with_state(pool);

    let port = 3000;
    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();

    println!("Listening at http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn test() -> &'static str {
    "Hello World!"
}
