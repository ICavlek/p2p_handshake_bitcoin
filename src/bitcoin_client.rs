use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::connection::Connection;

pub struct BitcoinClient {
    pub connection: Connection,
}

impl BitcoinClient {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<BitcoinClient, anyhow::Error> {
        let socket = tokio::time::timeout(Duration::from_millis(500), TcpStream::connect(addr))
            .await
            .unwrap()
            .unwrap();
        let (rx_stream, tx_stream) = socket.into_split();
        let connection = Connection::new(rx_stream, tx_stream);
        Ok(BitcoinClient { connection })
    }
}
