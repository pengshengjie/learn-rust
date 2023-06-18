use std::collections::HashMap;
use serde_json::Value;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let url = "http://127.0.0.1:3333/test";
    let resp = reqwest::get(url)
        .await?
        .json::<Value>()
        .await?;
    println!("{:#?}", resp["d"]);
    Ok(())
}