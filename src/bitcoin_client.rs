use bitcoin::{consensus::serialize, p2p::message::RawNetworkMessage};
use std::time::Duration;
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream, ToSocketAddrs,
};

use crate::{bitcoin_message::BitcoinMessage, connection::Connection};

pub struct BitcoinClient {
    connection: Connection<OwnedReadHalf, OwnedWriteHalf>,
}

impl BitcoinClient {
    pub async fn new<T: ToSocketAddrs>(addr: T) -> Result<BitcoinClient, anyhow::Error> {
        let socket = tokio::time::timeout(Duration::from_millis(500), TcpStream::connect(addr))
            .await
            .unwrap()
            .unwrap();
        let (rx_stream, tx_stream) = socket.into_split();
        let bitcoin_client = Self::connect(rx_stream, tx_stream)?;
        Ok(bitcoin_client)
    }

    pub fn connect(
        rx_stream: OwnedReadHalf,
        tx_stream: OwnedWriteHalf,
    ) -> Result<BitcoinClient, anyhow::Error> {
        let connection = Connection::new(rx_stream, tx_stream);
        Ok(BitcoinClient { connection })
    }

    pub async fn handshake(&mut self) -> Result<(), anyhow::Error> {
        let bitcoin_version_message = BitcoinMessage::version_message();
        self.handle_message(bitcoin_version_message).await?;
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        self.handle_message(bitcoin_verack_message).await?;
        Ok(())
    }

    pub async fn handle_message(
        &mut self,
        message: RawNetworkMessage,
    ) -> Result<(RawNetworkMessage, usize), anyhow::Error> {
        self.connection
            .write(serialize(&message).as_slice())
            .await?;
        match self.connection.read::<RawNetworkMessage>().await.unwrap() {
            Some((message, count)) => {
                println!("{:#?}, {}", message, count);
                Ok((message, count))
            }
            None => {
                println!("Empty buffer");
                Err(anyhow::anyhow!("Empty buffer"))
            }
        }
    }
}
