use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::{bitcoin_message::BitcoinMessage, connection::Connection};

pub struct BitcoinClient {
    connection: Connection,
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

    pub async fn handshake(&mut self) -> Result<(), anyhow::Error> {
        let bitcoin_message = BitcoinMessage::version_message();
        self.connection.write(bitcoin_message).await?;
        match self.connection.read().await.unwrap() {
            Some((message, count)) => println!("{:#?}, {}", message, count),
            None => println!("No message received"),
        };
        Ok(())
    }
}
