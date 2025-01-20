#![allow(unused)]
use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html,IntoResponse};
use axum::{routing::get,Router};

use tokio::net::TcpListener;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2));

    // region:      --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("--> LISTENING on {addr}\n");

    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, routes_hello).await.unwrap();
    // endregion:   --- Start Server
}

// region:      --- Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// ex: `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// ex: `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion:   --- Handler Hello

