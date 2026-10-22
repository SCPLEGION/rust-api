use std::array;

use axum::{
    Router, http::{StatusCode}, routing::{get},Json
};
use serde::{Serialize,Deserialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(test))
        .fallback(fallbackrt);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let server = axum::serve(listener, app).await;
    if server.is_err() {
        panic!("abort")
    }
}

async fn fallbackrt() -> String {
    StatusCode::NOT_FOUND.as_str().to_string()
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    lvl: i32,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    lvl: i32,
}

async fn test(Json(_payload): Json<CreateUser>) ->  String {
    let _name: String = _payload.name.to_string();
    let _number: i32 = _payload.lvl;
    let _user: User = User {name:_name,lvl:_number};
    let response: String = serde_json::to_string(&_user).unwrap(); 
    return response;
}