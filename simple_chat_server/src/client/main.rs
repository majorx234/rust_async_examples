use tokio::{io::{AsyncReadExt, AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream, time};
use tokio::io;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080");
    let mut stream = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        stream ).await.expect("cannot connect").expect("connection timeout");

    let (mut stream_reader, mut stream_writer) = stream.into_split();
    stream_writer.write_all(b"Hello, Server!\n").await?;

    let mut stream_buf_reader = BufReader::new(stream_reader);
    let server_read_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            let n = stream_buf_reader.read_line(&mut line).await?;
            if n == 0 {
                // Server closed connection
                eprintln!("Server disconnected.");
                break;
            }
             println!("Server response: {}", line);
        }
        io::Result::Ok(())
    });
    let input_read_task = tokio::spawn(async move {
        let mut stdin_buf_reader = BufReader::new(io::stdin());
        let mut input = String::new();

        loop {
            input.clear();
            let n = stdin_buf_reader.read_line(&mut input).await?;
            if n == 0 {
                // EOF on stdin (Ctrl+D)
                break;
            }

            // Send to server (newline-delimited protocol)
            stream_writer.write_all(input.as_bytes()).await?;
            stream_writer.flush().await?;
        }
        io::Result::Ok(())
    });
    tokio::select! {
        res = server_read_task => {
            res.expect("server_task panicked")?;
        }
        res = input_read_task => {
            res.expect("input_read_task panicked")?;
        }
    }

    Ok(())
}
