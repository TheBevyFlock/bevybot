use axum::{http::{HeaderMap, StatusCode}, routing::post, Router};
use tokio::net::TcpListener;

const ADDRESS: (&str, u16) = ("localhost", 8000);

#[tokio::main]
async fn main() {
    let app = Router::new().route("/webhook", post(webhook)).fallback(fallback);

    let listener = TcpListener::bind(ADDRESS).await.unwrap();

    println!("Listening on {:?}", ADDRESS);

    axum::serve(listener, app).await.unwrap();
}

async fn webhook(headers: HeaderMap, body: String) -> (StatusCode, &'static str) {
    println!("HEADERS={headers:?}");
    println!("BODY={body}");
    (StatusCode::ACCEPTED, "Accepted")
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not found :(")
}
