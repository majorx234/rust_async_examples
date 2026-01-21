use std::time::Instant;

use reqwest::{Error, StatusCode};
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let (status_1, status_2) = tokio::join!(
        get_status("https://duckduckgo.com"),
        get_status("https://lwn.net")
    );
    println!("status 1:{}",status_1.unwrap());
    println!("status 2:{}",status_2.unwrap());
    println!("Overall execution time: {}ms", start_time.elapsed().as_millis());
    Ok(())
}

async fn get_status(url:&str) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let status_code = reqwest::get(url).await?.status();
    let duration: u128 = start_time.elapsed().as_millis();
    println!("Took {}ms to fetch url '{}'", duration, url);
    Ok(status_code)
}
