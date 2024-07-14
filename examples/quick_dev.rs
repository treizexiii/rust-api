use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    let req_login = client.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_task = client.do_post(
        "/api/rpc",
        json!({
            "id":1,
            "method": "create_task",
            "params": {
                "data": {
                    "title" : "task AAA"
                }
            }
        }),
    );
    req_create_task.await?.print().await?;

    let req_update_task = client.do_post(
        "/api/rpc",
        json!({
            "id":1,
            "method": "update_task",
            "params": {
                "id": 1000,
                "data": {
                    "title" : "task BBB"
                }
            }
        }),
    );
    req_update_task.await?.print().await?;

    let req_delete_task = client.do_post(
        "/api/rpc",
        json!({
            "id":1,
            "method": "delete_task",
            "params": {
                "id": 1001
            }
        }),
    );
    req_delete_task.await?.print().await?;

    let req_list_task = client.do_post(
        "/api/rpc",
        json!({
            "id":1,
            "method": "list_task"
        }),
    );
    req_list_task.await?.print().await?;

    let req_logout = client.do_post(
        "/api/logout",
        json!({
            "logout": true,
        }),
    );
    req_logout.await?.print().await?;

    // let req_create_ticket = client.do_post(
    //     "/api/tickets",
    //     json!({
    //         "title": "Ticket AAA"
    //     })
    // );
    // req_create_ticket.await?.print().await?;
    //
    // client.do_delete("/api/tickets/1").await?.print().await?;
    //
    // client.do_get("/api/tickets").await?.print().await?;
    //
    // client.do_get("/not-found").await?.print().await?;
    //
    // client.do_get("/index.html").await?.print().await?;

    Ok(())
}
