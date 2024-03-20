#![allow(unused)]

use anyhow::Result;
use serde_json::json;
use httpc_test::*;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let req_login = hc.do_post("/api/login", {
        json!({
            "username": "admin",
            "password": "admin"
        })
    });

   req_login.await?.print().await?;


    let req_create_ticket = hc.do_post("/api/tickets", json!({"title": "TICKETER GOD"}));
    req_create_ticket.await?.print().await?;

    let req_list_tickets = hc.do_get("/api/tickets");
    req_list_tickets.await?.print().await?;

    let req_delete_ticket = hc.do_delete("/api/tickets/0");
    req_delete_ticket.await?.print().await?;

    let req_list_tickets = hc.do_get("/api/tickets");
    req_list_tickets.await?.print().await?;

    Ok(())

}


