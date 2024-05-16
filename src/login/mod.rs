mod temp_server;
use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::info;

use self::temp_server::service;

pub fn login() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    Ok(runtime.block_on(service())?)
}

async fn get_url() -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get("http://127.0.0.1:8000/api/login").send().await?;
    let data = res.json::<Value>().await?;
    open::that(data["url"].as_str().ok_or(anyhow!("url not found"))?)?;
    info!(
        "如果没有自动打开浏览器，请在浏览器中打开该地址进行登录：{}",
        data["url"]
    );
    Ok(())
}
