#![allow(unused)]

pub use self::error::{Error, Result};
use crate::model::{ModelController, Ticket, TicketForCreate};

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html,IntoResponse,Response};
use axum::routing::{get, get_service};
use axum::{Router, middleware};

use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;
use serde::Deserialize;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()>{
    // Initialize ModelController
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", web::routes_tickets::routes(mc.clone()))
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    // region:      --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("--> LISTENING on {addr}\n");

    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, routes_all).await.unwrap();
    // endregion:   --- Start Server
    
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
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

