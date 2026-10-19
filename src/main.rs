use axum::{
    Router, http::{StatusCode}, routing::{get},
};
use serde::{Serialize,Deserialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/usesr", get(test))
        .fallback(fallbackrt);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let server = axum::serve(listener, app).await;
    if server.is_err() {
        panic!("abort")
    }
}

async fn fallbackrt() ->&'static str {
    StatusCode::NOT_FOUND.as_str()
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize, Deserialize)]
struct User {
    name: &'static str,
    lvl: i32,
}

async fn test() -> &'static String {
    let _name: &str = "user";
    let _number: i32 = 1;
    let _user: User = User {name:_name,lvl:_number};
    let response = serde_json::to_string(&_user).unwrap(); 
    return response;
}