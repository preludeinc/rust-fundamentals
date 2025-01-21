#![allow(unused)]

use anyhow::Result;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080");
    // hc.expect("HELLO").do_get("/hello?name=Jen").await?.print().await?;
    // hc.expect("MAIN").do_get("/src/main.rs").await?.print().await?;

    hc.as_ref().expect("HELLO").do_get("/hello2/Mike").await?.print().await?;

    let binding = hc.as_ref().expect("LOGIN");
    let req_login = binding.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );
    req_login.await?.print().await?;

    let binding = hc.as_ref().expect("SEND TICKET");
    let req_create_ticket = binding.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.as_ref().expect("TICKETS").do_get("/api/tickets").await?.print().await?;

    Ok(())
}