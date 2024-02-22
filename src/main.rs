use p2p_handshake_bitcoin::bitcoint_client::BitcoinClient;

#[tokio::main]
async fn main() {
    let dest_uri = "45.9.148.241:8333";
    let timeout = 200;
    let bitcoin_client = BitcoinClient::new(dest_uri.to_string(), timeout);
    bitcoin_client.handshake().await;
}
