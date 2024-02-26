use std::fmt::Debug;

use anyhow::Context;
use bitcoin::{
    consensus::serialize,
    p2p::message::{NetworkMessage, RawNetworkMessage},
};
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
    #[error("Message error: Returned message content is not valid")]
    MessageError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl<Reader, Writer> BitcoinClient<Reader, Writer>
where
    Reader: AsyncReadExt + Unpin + Debug,
    Writer: AsyncWriteExt + Unpin + Debug,
{
    #[tracing::instrument(name = "Init Client", skip(rx_stream, tx_stream))]
    pub fn new(rx_stream: Reader, tx_stream: Writer) -> BitcoinClient<Reader, Writer> {
        let connection = Connection::new(rx_stream, tx_stream);
        BitcoinClient { connection }
    }

    #[tracing::instrument(name = "Handshake", skip(self))]
    pub async fn handshake(&mut self) -> Result<(), BitcoinClientError> {
        let bitcoin_version_message = BitcoinMessage::version_message();
        let (message, count) = self
            .handle_message(bitcoin_version_message)
            .await
            .context("Failed to handle version message")?;
        self.verify_version_message(message, count)
            .context("Failed to verify version message")?;
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        let (message, count) = self
            .handle_message(bitcoin_verack_message)
            .await
            .context("Failed to handle verack message")?;
        self.verify_verack_message(message, count)
            .context("Failed to verify verack message")?;
        Ok(())
    }

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

    fn verify_version_message(
        &self,
        message: RawNetworkMessage,
        count: usize,
    ) -> Result<(), BitcoinClientError> {
        if count != 126 {
            return Err(BitcoinClientError::MessageError);
        };
        let version_message = match message.payload() {
            NetworkMessage::Version(message) => message,
            _ => return Err(BitcoinClientError::MessageError),
        };
        if version_message.version < 7000 {
            return Err(BitcoinClientError::MessageError);
        }
        Ok(())
    }

    fn verify_verack_message(
        &self,
        message: RawNetworkMessage,
        count: usize,
    ) -> Result<(), BitcoinClientError> {
        if count != 24 {
            return Err(BitcoinClientError::MessageError);
        };
        match message.payload() {
            NetworkMessage::Verack => Ok(()),
            _ => Err(BitcoinClientError::MessageError),
        }
    }
}
