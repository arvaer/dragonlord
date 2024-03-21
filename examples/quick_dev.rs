#![allow(unused)]

use anyhow::Result;
use serde_json::json;
use httpc_test::*;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post("/api/login", {
        json!({
            "username": "admin",
            "pwd": "admin"
        })
    });

   req_login.await?.print().await?;



    Ok(())

}


