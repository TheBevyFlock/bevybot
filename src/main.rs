mod handler;
mod schema;

use axum::{
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use schema::IssueComment;
use tokio::net::TcpListener;

const ADDRESS: (&str, u16) = ("localhost", 8000);

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/webhook", post(webhook))
        .fallback(fallback);

    let listener = TcpListener::bind(ADDRESS).await.unwrap();

    println!("Listening on {:?}", ADDRESS);

    axum::serve(listener, app).await.unwrap();
}

async fn webhook(
    // TODO: Verify Github headers.
    _headers: HeaderMap,
    // TODO: Deserialize more than just issue comments.
    Json(issue_comment): Json<IssueComment>,
) -> (StatusCode, &'static str) {
    tokio::spawn(handler::handle(issue_comment));
    (StatusCode::ACCEPTED, "Accepted")
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not found :(")
}
