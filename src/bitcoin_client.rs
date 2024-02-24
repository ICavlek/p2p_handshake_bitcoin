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

#[derive(thiserror::Error, Debug)]
pub enum BitcoinClientError {
    #[error("Communication error: Wrong response")]
    CommunicationError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
    pub async fn handshake(&mut self) -> Result<(), BitcoinClientError> {
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
    ) -> Result<(RawNetworkMessage, usize), BitcoinClientError> {
        self.connection
            .write(serialize(&message).as_slice())
            .await?;
        let response = match self.connection.read::<RawNetworkMessage>().await {
            Ok(response) => response,
            Err(_) => return Err(BitcoinClientError::CommunicationError),
        };
        match response {
            Some((message, count)) => Ok((message, count)),
            None => Err(BitcoinClientError::UnexpectedError(anyhow::anyhow!(
                "Empty buffer returned"
            ))),
        }
    }
}
