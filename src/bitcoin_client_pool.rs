use std::collections::HashMap;

use tokio::task::JoinHandle;

use crate::{bitcoin_client::BitcoinClient, stream::Stream};

pub struct BitcoinClientPool {
    map_of_tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>>,
}

impl BitcoinClientPool {
    pub fn new(nodes: Vec<String>) -> BitcoinClientPool {
        let mut map_of_tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>> =
            HashMap::new();
        for node in nodes {
            let task = tokio::task::spawn(BitcoinClientPool::perform_handshake(node.clone()));
            map_of_tasks.insert(node, task);
        }
        Self { map_of_tasks }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        for (node, task) in self.map_of_tasks.into_iter() {
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

    async fn perform_handshake(uri: String) -> Result<(), anyhow::Error> {
        let stream = match Stream::new(&uri).await {
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
