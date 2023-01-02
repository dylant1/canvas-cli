use reqwest;
use std::env;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {

    let access_token: &'static str = env!("ACCESS_TOKEN");
    let url ="https://canvas.instructure.com/api/v1/courses?access_token=".to_owned() + access_token;

    let body = reqwest::get(url).await?.text().await?;

    println!("Body: {}", body);
    println!("TOKEN: {}", access_token);
    
    Ok(())
}
