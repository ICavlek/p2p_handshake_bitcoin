use p2p_handshake_bitcoin::{bitcoin_client::BitcoinClient, bitcoin_message::BitcoinMessage};

#[tokio::main]
async fn main() {
    let mut bitcoin_client = BitcoinClient::connect("45.9.148.241:8333").await.unwrap();
    let bitcoin_message = BitcoinMessage::version_message();
    let _ = bitcoin_client.connection.write(bitcoin_message).await;
    match bitcoin_client.connection.read().await.unwrap() {
        Some((message, count)) => println!("{:#?}, {}", message, count),
        None => println!("No message received"),
    };
}
