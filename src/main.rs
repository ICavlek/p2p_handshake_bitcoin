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
    let bitcoin_client_pool = BitcoinClientPool::new(vec![
        "45.9.148.241:8333".to_string(),
        "95.105.172.171:8333".to_string(),
        "46.17.99.26:8333".to_string(),
    ]);
    bitcoin_client_pool.run().await?;
    Ok(())
}
