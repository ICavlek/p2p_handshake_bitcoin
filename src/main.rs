use p2p_handshake_bitcoin::bitcoin_client::BitcoinClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut bitcoin_client = BitcoinClient::connect("45.9.148.241:8333").await.unwrap();
    bitcoin_client.handshake().await?;
    Ok(())
}
