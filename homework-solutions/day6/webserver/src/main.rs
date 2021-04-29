use std::env;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read PORT from environment, default to 8080
    let port = match env::var("PORT") {
        Ok(x) => x,
        Err(_) => "8080".to_string(),
    };
    let address = "127.0.0.1:".to_string()+&port;
    let listener = TcpListener::bind(address).await?;
    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                eprintln!("failed to accept connection. err = {:?}", e);
                continue;
            }
        };
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            // Write response
            let response;
            let get = b"GET / HTTP/1.1\r\n";
            if buf.starts_with(get) {
                response = "HTTP/1.1 200 OK\r\n\r\n";
            } else {
                response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            }
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return;
            }
        });
    }
}
