use std::env;
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let day = 1;
    let input = get_input(day)
        .await
        .unwrap();
    println!("{}", input)
}

async fn get_input(day: i32) -> Result<String, Box<dyn std::error::Error>> {
    let session = env::var("SESSION").unwrap();
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .header("Cookie", format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;
    Ok(resp)
}