
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // region: -- Start server --
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Geremy").await?.print().await?;
    

    // hc.do_get("/Cargo.toml").await?.print().await?;

    let req_login = hc.do_post("/api/login", json!({
        "username": "admin",
        "pwd": "admin"
    }));
    req_login.await?.print().await?;

    hc.do_get("/hello2/John").await?.print().await?;



    Ok(())
}