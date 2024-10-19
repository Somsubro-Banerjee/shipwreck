use common::{Message, serialize_message};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    // Example message
    let message = Message::new("key1", "This is a message from the producer");

    // Serialize message
    let serialized_msg = serialize_message(&message);

    // Send to broker (assuming broker is listening on localhost:8080)
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            stream.write_all(&serialized_msg).await.expect("Failed to send message");
            println!("Producer: Message sent successfully!");
        }
        Err(e) => {
            println!("Producer: Failed to connect to broker - {}", e);
        }
    }
}
