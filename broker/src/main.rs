use common::{deserialize_message, serialize_message, Message};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind broker listener");
    let consumers: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Broker: Waiting for producers and consumers...");

    loop {
        let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
        let consumers = Arc::clone(&consumers);

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            let n = socket.read(&mut buffer).await.expect("Failed to read message");

            if n > 0 {
                let message: Message = deserialize_message(&buffer[..n]);
                println!("Broker: Received message: {:?}", message);

                // Forward the message to all consumers
                let serialized_msg = serialize_message(&message);
                let mut consumers = consumers.lock().await; // Use await to lock the async mutex
                for consumer in consumers.iter_mut() {
                    let _ = consumer.write_all(&serialized_msg).await;
                }
            }
        });
    }
}
