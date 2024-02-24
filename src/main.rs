use p2p_handshake_bitcoin::{
    bitcoin_client::BitcoinClient,
    stream::Stream,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(
        "p2p_handshake_bitcoin".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    let stream = Stream::new().await;
    let mut bitcoin_client = BitcoinClient::new(stream.rx, stream.tx).unwrap();
    bitcoin_client.handshake().await?;
    Ok(())
}
