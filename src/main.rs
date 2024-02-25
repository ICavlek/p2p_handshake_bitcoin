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

    let stream = match Stream::new("45.9.148.241:8333").await {
        Ok(stream) => stream,
        Err(e) => {
            return {
                tracing::error!("Failed to initialize TCP stream");
                Err(e)
            }
        }
    };
    let mut bitcoin_client = BitcoinClient::new(stream.rx, stream.tx);
    match bitcoin_client.handshake().await {
        Ok(()) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to perform handshake: {}", e);
            Err(anyhow::anyhow!(e))
        }
    }
}
