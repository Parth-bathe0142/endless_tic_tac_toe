use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sqlx::{MySqlPool, prelude::*};
use tokio::net::TcpListener;

#[derive(FromRow, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect("mysql://root:admin@localhost:3306/tic_tac_toe")
        .await
        .unwrap();

    let app = Router::new()
        .route("/test", get(test))
        .route("/users", get(test_sql))
        .with_state(pool);

    let port = 3000;
    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();

    println!("Listening at http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn test() -> &'static str {
    "Hello World!"
}

async fn test_sql(State(pool): State<MySqlPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("select id, `name` from Users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(users)
}
