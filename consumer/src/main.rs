use common::{deserialize_message, serialize_message, Message};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

#[tokio::main]
async fn main() {
    // Connect to the broker
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.expect("Failed to connect to broker");

    println!("Consumer: Connected to broker. Waiting for messages...");

    loop {
        let mut buffer = vec![0; 1024]; // Buffer to read incoming messages
        match socket.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let message: Message = deserialize_message(&buffer[..n]);
                println!("Consumer: Received message: {:?}", message);
            }
            Ok(_) => {
                // Connection closed
                println!("Consumer: Connection closed.");
                break;
            }
            Err(e) => {
                println!("Consumer: Failed to read message: {:?}", e);
                break;
            }
        }
    }
}
