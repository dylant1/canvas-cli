use reqwest;
use clap::Parser;
use std::env;
use mini_redis::Result;

#[derive(Parser)]
struct Cli {
    cmd: String,
}

struct User {
    access_token: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Cli::parse();
    let user = User {
        access_token: env!("ACCESS_TOKEN"),
    };

    //for _ in 0..args.count { 
        //println!("hello: {}", args.cmd);
    //}

    println!("COMMAND: {} ", args.cmd);
    let access_token: &'static str = env!("ACCESS_TOKEN");

    let grades: String = get_grades(user.access_token).await.unwrap();
    println!("{:?}", grades);

    
    Ok(())
}

async fn get_grades(access_token: &'static str) -> Result<String> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://canvas.instructure.com/api/v1/courses")
        .header("Authorization", "Bearer ".to_owned() + access_token)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}
