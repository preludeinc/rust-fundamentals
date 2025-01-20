#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080");

    //hc.expect("REASON").do_get("/hello?name=Jen").await?.print().await?;
    hc.expect("REASON").do_get("/hello2/Mike").await?.print().await?;

    Ok(())
}