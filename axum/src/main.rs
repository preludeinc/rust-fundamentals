
#![allow(unused)]
use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    response::Html,
    };
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region:      --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("--> LISTENING on {addr}\n");

    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, routes_hello).await.unwrap();
    // endregion:   --- Start Server

    // region:      --- Handler Hello
    async fn handler_hello() -> impl IntoResponse {
        println!("->> {:<12} - handler_hello", "HANDLER");
        Html("Hello <strong>World!!</strong>")
    }
    // endregion:   --- Handler Hello
}

