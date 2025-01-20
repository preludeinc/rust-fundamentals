#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080");
    let hc2 = httpc_test::new_client("http://localhost:8080");

    //hc.expect("REASON").do_get("/hello?name=Jen").await?.print().await?;
    //hc.expect("REASON").do_get("/hello2/Mike").await?.print().await?;

    hc.expect("MAIN").do_get("/src/main.rs").await?.print().await?;

    let binding = hc2.expect("LOGIN");
    let req_login = binding.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );
    req_login.await?.print().await?;

    Ok(())
}