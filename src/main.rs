use p2p_handshake_bitcoin::{bitcoint_client::BitcoinClient, client::Client};

#[tokio::main]
async fn main() {
    let dest_uri = "45.9.148.241:8333";
    let timeout = 200;
    let bitcoin_client = BitcoinClient::new(dest_uri.to_string(), timeout);
    //bitcoin_client.handshake().await;

    let mut client = Client::connect("45.9.148.241:8333").await.unwrap();
    let bicoin_message = bitcoin_client.get_default_version_message();
    let _ = client.connection.write(bicoin_message).await;
    match client.connection.read().await.unwrap() {
        Some((message, count)) => println!("{:#?}, {}", message, count),
        None => println!("No message received"),
    };
}
