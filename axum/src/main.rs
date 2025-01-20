#![allow(unused)]
mod error;
mod web;

pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html,IntoResponse};
use axum::routing::{get, get_service};
use axum::{Router};

use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .fallback_service(routes_static());

    // region:      --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("--> LISTENING on {addr}\n");

    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, routes_all).await.unwrap();
    // endregion:   --- Start Server
}

fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
}

// region:      -- Routes Hello
fn routes_hello() -> Router {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2));
    routes_hello
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

