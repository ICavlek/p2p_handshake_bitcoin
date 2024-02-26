use p2p_handshake_bitcoin::{
    bitcoin_client_pool::BitcoinClientPool,
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
    BitcoinClientPool::run().await?;
    Ok(())
}
