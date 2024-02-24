use std::time::Duration;

use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};

pub struct Stream {
    pub rx: OwnedReadHalf,
    pub tx: OwnedWriteHalf,
}

impl Stream {
    pub async fn new() -> Self {
        let socket = tokio::time::timeout(
            Duration::from_millis(500),
            TcpStream::connect("45.9.148.241:8333"),
        )
        .await
        .unwrap()
        .unwrap();
        let (rx, tx) = socket.into_split();
        Self { rx, tx }
    }
}
