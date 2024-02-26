use clap::Parser;

use p2p_handshake_bitcoin::{
    bitcoin::client_pool::BitcoinClientPool,
    parser_arguments::Arguments,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let subscriber = get_subscriber(
        "p2p_handshake_bitcoin".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    let bitcoin_client_pool = BitcoinClientPool::new(args.uri_nodes, args.timeout);
    bitcoin_client_pool.run().await?;
    Ok(())
}
