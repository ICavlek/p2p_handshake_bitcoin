use p2p_handshake_bitcoin::{bitcoin_client::BitcoinClient, stream::Stream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = Stream::new().await;
    let mut bitcoin_client = BitcoinClient::new(stream.rx, stream.tx).unwrap();
    bitcoin_client.handshake().await?;
    Ok(())
}
