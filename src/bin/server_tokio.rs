use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port_number = 12345;
    let listener = TcpListener::bind(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port_number,
    ))
    .await
    .expect("Failed to bind to address");

    println!("Listening for connections on port {}", port_number);

    match listener.accept().await {
        Ok((mut socket, addr)) => {
            println!("Accepted connection from {:?}", addr);

            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(bytes_read) => {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Received message is \"{}\"", message);
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    std::process::exit(1);
                }
            }

            let response = "Hello from server";
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Error sending message to client: {}", e);
                std::process::exit(1);
            } else {
                println!("Message sent to client");
            }
        }
        Err(e) => {
            eprintln!("Accept error: {}", e);
            std::process::exit(1);
        }
    }
}
