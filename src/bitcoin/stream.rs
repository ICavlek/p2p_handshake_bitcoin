use std::time::Duration;

use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};

/// Module that provides reading and writing streams
pub struct Stream {
    pub rx: OwnedReadHalf,
    pub tx: OwnedWriteHalf,
}

impl Stream {
    /// Creates a stream on the provided ip addresses of Bitcoin nodes
    pub async fn new(uri: &str, timeout: u64) -> Result<Self, anyhow::Error> {
        let socket =
            tokio::time::timeout(Duration::from_millis(timeout), TcpStream::connect(uri)).await??;
        let (rx, tx) = socket.into_split();
        Ok(Self { rx, tx })
    }
}
