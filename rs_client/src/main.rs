use tokio::net::TcpSocket;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:5252".parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;
    stream.write_all(b"GET test").await?;
    stream.flush().await?;
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;
    let message = String::from_utf8(buffer[..size].to_vec())?;
    println!("{}", message);
    Ok(())
}
