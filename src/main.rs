use reqwest;
use clap::Parser;
use std::env;
use mini_redis::Result;

#[derive(Parser)]
struct Cli {
    cmd: String,
}


#[tokio::main]
async fn main() -> Result<()> {

    let args = Cli::parse();

    //for _ in 0..args.count { 
        //println!("hello: {}", args.cmd);
    //}
    println!("COMMAND: {} ", args.cmd);

    let access_token: &'static str = env!("ACCESS_TOKEN");
    let url ="https://canvas.instructure.com/api/v1/courses?access_token=".to_owned() + access_token;


    let body = reqwest::get(url).await?.text().await?;

    println!("Body: {}", body);
    println!("TOKEN: {}", access_token);
    
    Ok(())
}

async fn get_grades() {

}
