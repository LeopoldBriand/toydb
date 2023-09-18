mod file_manager;
mod query_parser;
use std::env;
use std::error::Error;

use file_manager::Manager;
use query_parser::parse;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:5252".to_string());
    let listener = TcpListener::bind(&addr).await?;
    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let env_path = env::current_dir().unwrap();
            let mut manager = Manager::new(&env_path);
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                let query = String::from_utf8(buf.clone()).unwrap();
                let response = format!("{}",parse(&mut manager, &query).unwrap());
                socket
                    .write_all( response.as_bytes())
                    .await
                    .expect("failed to write data to socket");
                
            }
        });
    }
    
}
