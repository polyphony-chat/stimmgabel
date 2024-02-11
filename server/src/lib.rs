use axum::{
    routing::{get, post, put},
    Router,
};

pub async fn run(_port: u16) {
    let server = Router::new()
        .route("/", get(hello_world))
        .route("/p2core/session/trust", post(hello_world))
        .route("/p2core/session/idcert", post(hello_world))
        .route("/p2core/keypackage/@me", post(hello_world))
        .route("/p2core/keypackage_lr", put(hello_world))
        .route("/p2core/register", post(hello_world))
        .route("/p2core/challenge", get(hello_world))
        .route("/p2core/session/identify", post(hello_world))
        .route("/p2core/session/revoke", put(hello_world))
        .route("/p2core/key/server", put(hello_world))
        .route("/p2core/key/server", get(hello_world))
        .route("/p2core/key/actor/:fid", get(hello_world))
        .route("/p2core/session/idcert/extern", post(hello_world))
        .route("/p2core/keypackage/:fid", get(hello_world))
        .route("/p2core/events", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server).await.unwrap();
}

async fn hello_world() -> String {
    String::from("hello, world!")
}
