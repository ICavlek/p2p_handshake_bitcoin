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
        let bitcoin_version_message = BitcoinMessage::version_message();
        self.connection
            .write(serialize(&bitcoin_version_message).as_slice())
            .await?;
        match self.connection.read::<RawNetworkMessage>().await.unwrap() {
            Some((message, count)) => println!("{:#?}, {}", message, count),
            None => println!("No message received"),
        };

        let bitcoin_verack_message = BitcoinMessage::verack_message();
        self.connection
            .write(serialize(&bitcoin_verack_message).as_slice())
            .await?;
        match self.connection.read::<RawNetworkMessage>().await.unwrap() {
            Some((message, count)) => println!("{:#?}, {}", message, count),
            None => println!("No message received"),
        };
        Ok(())
    }
}
