use std::collections::HashMap;

use tokio::task::JoinHandle;

use crate::{bitcoin::client::BitcoinClient, bitcoin::stream::Stream};

pub struct BitcoinClientPool {
    tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>>,
}

impl BitcoinClientPool {
    pub fn new(nodes: Vec<String>, timeout: u64) -> BitcoinClientPool {
        let mut tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>> = HashMap::new();
        for node in nodes {
            let task =
                tokio::task::spawn(BitcoinClientPool::perform_handshake(node.clone(), timeout));
            tasks.insert(node, task);
        }
        Self { tasks }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        for (node, task) in self.tasks.into_iter() {
            let result = match task.await {
                Ok(result) => result,
                Err(e) => {
                    tracing::error!(
                        error.cause_chain = ?e,
                        error.message = %e,
                    );
                    Err(anyhow::anyhow!(e))
                }
            };
            match result {
                Ok(()) => {
                    tracing::info!("Successfully performed handshake for Node {}", node);
                }
                Err(e) => {
                    tracing::error!(error.cause_chain = ?e, error.message = %e,"Error with Node {}", node);
                }
            }
        }
        Ok(())
    }

    #[tracing::instrument("Performing handshake", skip(timeout))]
    async fn perform_handshake(uri: String, timeout: u64) -> Result<(), anyhow::Error> {
        let stream = match Stream::new(&uri, timeout).await {
            Ok(stream) => stream,
            Err(e) => {
                return {
                    tracing::error!("Failed to initialize TCP stream");
                    Err(anyhow::anyhow!("Failed to initialize TCP stream, {}", e))
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
