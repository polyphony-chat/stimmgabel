use axum::{routing::get, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server).await.unwrap();
}

async fn hello_world() -> String {
    String::from("hello, world!")
}
