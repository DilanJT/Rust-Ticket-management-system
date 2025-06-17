
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // region: -- Start server --
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Geremy").await?.print().await?;
    hc.do_get("/hello2/John").await?.print().await?;



    Ok(())
}