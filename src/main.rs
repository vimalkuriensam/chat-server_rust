use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    let (socket, _addr) = listener.accept().await.unwrap();
}
