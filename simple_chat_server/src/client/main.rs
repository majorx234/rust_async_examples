use tokio::{io::{AsyncReadExt, AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream, time};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080");
    let mut stream = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        stream ).await.expect("cannot connect").expect("connection timeout");
    stream.write_all(b"Hello, Server!\n").await?;
    let mut buffer_reader = BufReader::new(stream);
    let mut line = String::new();
    let n = buffer_reader.read_line(&mut line).await?;
    println!("Server response: {}", line);

    Ok(())
}
