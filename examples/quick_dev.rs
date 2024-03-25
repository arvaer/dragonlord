#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome2"
        }),
    );
    req_login.await?.print().await?;
    let req_tas_create = hc.do_post(
        "/api/rpc",
        json!({
            "id": "1",
            "method" : "create_task",
            "params" : {
                "data" : {
                    "title" : "task AAA"
                }
            }
        }),
    );
    req_tas_create.await?.print().await?;

    let req_tas_create = hc.do_post(
        "/api/rpc",
        json!({
            "id": "2",
            "method" : "create_task",
            "params" : {
                "data" : {
                    "title" : "task AAA2"
                }
            }
        }),
    );
    req_tas_create.await?.print().await?;

    let req_list_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id" : 1,
            "method": "list_tasks"
        }),
    );


    let req_list_delete = hc.do_post(
        "/api/rpc",
        json!({
            "id" : "6",
            "method" : "delete_task",
            "params" : {
                    "id": 1002
            }
        }),
    );

    req_list_delete.await?.print().await?;

    let req_list_update = hc.do_post("/api/rpc", json!({
        "id" : 8,
        "method": "update_task",
        "params":{
            "id":1002,
            "data" : {
                "title" : "John Mackloes"
            }
        }
    }));

    req_list_update.await?.print().await?;
    req_list_tasks.await?.print().await?;

    let req_logoff = hc.do_post(
        "/api/logout",
        json!({
            "logoff": true
        }),
    );
    req_logoff.await?.print().await?;

    Ok(())
}
