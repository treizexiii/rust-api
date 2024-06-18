
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/hello").await?.print().await?;

    client.do_get("/hello?name=john").await?.print().await?;

    client.do_get("/hello2/smith").await?.print().await?;

    let req_login = client.do_post(
        "/api/login",
        json!({
            "username": "demo",
            "password": "azerty"
        })
    );
    req_login.await?.print().await?;

    let req_create_ticket = client.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        })
    );
    req_create_ticket.await?.print().await?;

    // client.do_delete("/api/tickets/1").await?.print().await?;

    client.do_get("/api/tickets").await?.print().await?;

    client.do_get("/not-found").await?.print().await?;

    client.do_get("/index.html").await?.print().await?;

    Ok(())
}
