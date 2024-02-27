use std::collections::HashMap;

use tokio::task::JoinHandle;

use crate::{bitcoin::client::BitcoinClient, bitcoin::stream::Stream};

/// Module to handle multiple bitcoin client handshakes
pub struct BitcoinClientPool {
    tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>>,
}

impl BitcoinClientPool {
    /// Creates mutltiple bitcoin clients from the vector of uri's.
    /// Example shows localhost as uri, instead use real bitcoin node ip.
    ///
    /// #Example
    ///
    /// ```
    /// use p2p_handshake_bitcoin::bitcoin::client_pool::BitcoinClientPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients = vec![
    ///         "127.0.0.1:1".to_string(), "127.0.0.1:2".to_string(), "127.0.0.1:3".to_string()
    ///     ];
    ///     let timeout = 500; // miliseconds
    ///     let client_pool = BitcoinClientPool::new(clients, timeout);
    /// }
    /// ```
    pub fn new(nodes: Vec<String>, timeout: u64) -> BitcoinClientPool {
        let mut tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>> = HashMap::new();
        for node in nodes {
            let task =
                tokio::task::spawn(BitcoinClientPool::perform_handshake(node.clone(), timeout));
            tasks.insert(node, task);
        }
        Self { tasks }
    }

    /// Runs mutltiple bitcoin clients from the BitcoinClientPool.
    /// Example shows localhost as uri, instead use real bitcoin node ip.
    ///
    /// #Example
    ///
    /// ```
    /// use p2p_handshake_bitcoin::bitcoin::client_pool::BitcoinClientPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients = vec![
    ///         "127.0.0.1:1".to_string(), "127.0.0.1:2".to_string(), "127.0.0.1:3".to_string()
    ///     ];
    ///     let timeout = 500; // miliseconds
    ///     let client_pool = BitcoinClientPool::new(clients, timeout);
    ///     client_pool.run().await.unwrap();
    /// }
    /// ```
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

    /// Runst handshake on all provided bitcoin clients.
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
