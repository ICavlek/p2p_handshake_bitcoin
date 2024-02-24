use std::fmt::Debug;

use bitcoin::{consensus::serialize, p2p::message::RawNetworkMessage};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{bitcoin_message::BitcoinMessage, connection::Connection};

pub struct BitcoinClient<Reader, Writer>
where
    Reader: AsyncReadExt + Unpin,
    Writer: AsyncWriteExt + Unpin,
{
    connection: Connection<Reader, Writer>,
}

impl<Reader, Writer> BitcoinClient<Reader, Writer>
where
    Reader: AsyncReadExt + Unpin + Debug,
    Writer: AsyncWriteExt + Unpin + Debug,
{
    #[tracing::instrument(name = "Init Client", skip(rx_stream, tx_stream))]
    pub fn new(
        rx_stream: Reader,
        tx_stream: Writer,
    ) -> Result<BitcoinClient<Reader, Writer>, anyhow::Error> {
        let connection = Connection::new(rx_stream, tx_stream);
        Ok(BitcoinClient { connection })
    }

    #[tracing::instrument(name = "Handshake", skip(self))]
    pub async fn handshake(&mut self) -> Result<(), anyhow::Error> {
        let bitcoin_version_message = BitcoinMessage::version_message();
        self.handle_message(bitcoin_version_message).await?;
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        self.handle_message(bitcoin_verack_message).await?;
        Ok(())
    }

    #[tracing::instrument(name = "Handling message", skip(self, message))]
    pub async fn handle_message(
        &mut self,
        message: RawNetworkMessage,
    ) -> Result<(RawNetworkMessage, usize), anyhow::Error> {
        self.connection
            .write(serialize(&message).as_slice())
            .await?;
        match self.connection.read::<RawNetworkMessage>().await.unwrap() {
            Some((message, count)) => Ok((message, count)),
            None => Err(anyhow::anyhow!("Empty buffer")),
        }
    }
}
