use axum::{
    body::Body,
    routing::{get, post},
    response::Json,
    Router,
};
use axum::extract::{Path};
use serde_json::{Value, json};

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn user(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({ "users": id, "name":"Game" }))
}

async fn create_user(Json(payload): Json<serde_json::Value>)-> Json<Value> {
    let id = payload.get("id");
    let id = match payload["id"].as_i64() {
        Some(n) => n,
        None => -1
    };
    Json(json!({ "users": id, "name": "new" }))
}

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/plain_text", get(plain_text))
    .route("/json", get(json))
    .route(
        "/users/:id",
        get(user),
    )
    .route("/users", post(create_user));
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}