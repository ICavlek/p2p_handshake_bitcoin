use crate::{bitcoin_client::BitcoinClient, stream::Stream};

pub struct BitcoinClientPool;

impl BitcoinClientPool {
    pub async fn run() -> Result<(), anyhow::Error> {
        let task1 = tokio::task::spawn(BitcoinClientPool::perform_handshake("45.9.148.241:8333"));
        let task2 = tokio::task::spawn(BitcoinClientPool::perform_handshake("95.105.172.171:8333"));
        let mut handles = Vec::new();
        handles.push(task1);
        handles.push(task2);
        for (index, handle) in handles.into_iter().enumerate() {
            let result = handle.await?;
            match result {
                Ok(()) => {}
                Err(e) => {
                    tracing::error!(
                        error.cause_chain = ?e,
                        error.message = %e,
                        "Error in thread {}", index
                    );
                }
            }
        }
        Ok(())
    }

    async fn perform_handshake(uri: &str) -> Result<(), anyhow::Error> {
        let stream = match Stream::new(uri).await {
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
}
