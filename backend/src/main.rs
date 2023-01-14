use axum::{
    routing::{get},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use std::net::SocketAddr;
use axum::extract::Path;  
use serde_json::{json};

pub struct UrlParams<T>(pub T);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/v1/todos/:id", get(json_hello));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn json_hello(
    Path(id): Path<String>
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "id": id.as_str(),
            "message": "Just do it!",
            "priority": "A",
            "assigned": "user@example.com"
        }))
    )
}

// http://127.0.0.1:3000/v1/todos/63443a02-2137-48e8-8db5-79fa54e8bfdf

// {
//     "id": "63443a02-2137-48e8-8db5-79fa54e8bfdf",
//     "message": "Just do it!",
//     "priority": "A",
//     "assigned": "user@example.com"
// }