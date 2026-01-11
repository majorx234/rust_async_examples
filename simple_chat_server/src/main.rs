use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream}, sync::broadcast
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.expect("cannot bind  socket");
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(50);
    loop {
        let (mut socket, addr): (TcpStream, SocketAddr) = listener.accept().await.expect("cannot establish incomming connection");
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn( async move{
            let (mut sock_reader, mut sock_writer) = socket.split();
            let mut buf_reader = BufReader::new(&mut sock_reader);

            loop {
                let mut line = String::new();
                tokio::select!{
                    result = buf_reader.read_line(&mut line) => {
                        if result.expect("cannot read sock data") == 0 {
                            break;
                        }
                        let _ = tx.send((line.clone(), addr));
                    }
                    new_msgs = rx.recv() => {
                        let (new_msgs, addr_new) = new_msgs.expect("error in boradcast channel");
                        if addr != addr_new {
                            sock_writer.write_all(new_msgs.as_bytes()).await.expect("cannot write to sock data");
                        }
                    }
                }
                line.clear();
            }
        });
    }
}
