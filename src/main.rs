use std::time::Duration;

use p2p_handshake_bitcoin::bitcoin_client::BitcoinClient;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = tokio::time::timeout(
        Duration::from_millis(500),
        TcpStream::connect("45.9.148.241:8333"),
    )
    .await
    .unwrap()
    .unwrap();
    let (rx_stream, tx_stream) = socket.into_split();
    let mut bitcoin_client = BitcoinClient::new(rx_stream, tx_stream).unwrap();
    bitcoin_client.handshake().await?;
    Ok(())
}
