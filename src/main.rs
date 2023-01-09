use reqwest;
use clap::Parser;
use std::env;
use mini_redis::Result;

#[derive(Parser)]
struct Cli {
    cmd: String,
}

#[derive(Debug)]
struct Enrollment {
    enrollment_state: String,
    user_id: i64
}

#[derive(Debug)]
struct Class {
    account_id: i64,
    course_code: String,
    enrollments: Vec<Enrollment>
}
struct User {
    access_token: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {


    //TODO: Figure out how to serialize json
    let args = Cli::parse();
    let user = User {
        access_token: env!("ACCESS_TOKEN"),
    };

    //for _ in 0..args.count { 
        //println!("hello: {}", args.cmd);
    //}

    println!("COMMAND: {} ", args.cmd);
    let access_token: &'static str = env!("ACCESS_TOKEN");

    //let grades: String = get_grades(user.access_token).await.unwrap();
    //println!("{:?}", grades);

    let client = reqwest::Client::new();
    let body = client
        .get("https://canvas.instructure.com/api/v1/courses/")
        .header("Authorization", "Bearer ".to_owned() + access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", body);

    Ok(())
}

async fn get_grades(access_token: &'static str) -> Result<()> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://canvas.instructure.com/api/v1/courses/")
        .header("Authorization", "Bearer ".to_owned() + access_token)
        .send()
        .await?
        .json::<Class>()
        .await?;
    println!("{:#?}", body);
    println!("ETSTSETST");
    Ok(())
}
