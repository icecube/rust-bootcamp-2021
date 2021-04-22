use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;

fn main() {
    // read PORT from environment, default to 8080
    let port = match env::var("PORT") {
        Ok(x) => x,
        Err(_) => "8080".to_string(),
    };
    let address = "127.0.0.1:".to_string()+&port;
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
