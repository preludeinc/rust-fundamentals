#![allow(unused)]
use std::net::SocketAddr;
use axum::{
    extract::Query,
    response::IntoResponse,
    response::Html,
    routing::get,
    Router,
    };
use tokio::net::TcpListener;
use serde::Deserialize;

// region:      --- Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion:   --- Handler Hello

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region:      --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("--> LISTENING on {addr}\n");

    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, routes_hello).await.unwrap();
    // endregion:   --- Start Server
}

