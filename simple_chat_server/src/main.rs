use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream}
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.expect("cannot bind  sokcet");
    loop {
        let (mut socket, _addr): (TcpStream, SocketAddr) = listener.accept().await.expect("cannot establish incomming connection");
        tokio::spawn( async move{
            let (mut sock_reader, mut sock_writer) = socket.split();
            let mut buf_reader = BufReader::new(&mut sock_reader);
            let mut line = String::new();

            loop {
                let bytes_read: usize = buf_reader.read_line(&mut line).await.expect("cannot read sokc data");
                if bytes_read == 0 {
                    break;
                }
                sock_writer.write_all(line.as_bytes()).await.expect("cannot write on sock data");
                line.clear();
            }
        });
    }
}
