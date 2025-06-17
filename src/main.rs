#![allow(unused)]

use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new().merge(routes_hello());

    // region: -- Start server --
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}

// region: --routes hello --
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

// region: -- Handler hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// endregion: -- Handler Hello --

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}😀</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}😀</strong>"))
}
