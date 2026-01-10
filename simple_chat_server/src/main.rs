use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream}, sync::broadcast
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.expect("cannot bind  socket");
    let (tx, mut rx) = broadcast::channel::<String>(50);
    loop {
        let (mut socket, _addr): (TcpStream, SocketAddr) = listener.accept().await.expect("cannot establish incomming connection");
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn( async move{
            let (mut sock_reader, mut sock_writer) = socket.split();
            let mut buf_reader = BufReader::new(&mut sock_reader);

            loop {
                let mut line = String::new();
                let bytes_read: usize = buf_reader.read_line(&mut line).await.expect("cannot read sock data");
                if bytes_read == 0 {
                    break;
                }
                tx.send(line);
                let lines = rx.recv().await.expect("");
                sock_writer.write_all(lines.as_bytes()).await.expect("cannot write on sock data");
                //line.clear();
            }
        });
    }
}
