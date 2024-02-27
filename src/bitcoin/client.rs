use std::fmt::Debug;

use anyhow::Context;
use bitcoin::{
    consensus::serialize,
    p2p::message::{NetworkMessage, RawNetworkMessage},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{bitcoin::connection::Connection, bitcoin::message::BitcoinMessage};

/// Client that is used to establish communication with the remote node.
pub struct BitcoinClient<Reader, Writer>
where
    Reader: AsyncReadExt + Unpin,
    Writer: AsyncWriteExt + Unpin,
{
    connection: Connection<Reader, Writer>,
}

/// Error enumeration to represent higher abstraction level of errors.
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
    /// Creates a Bitcoin client based on provided reading and writing streams.
    /// Use [Stream] module as the basis.
    /// Example shows localhost ip address and port, instead use real bitcoin node ip.
    ///
    /// [Stream]: crate::bitcoin::stream::Stream
    /// # Example
    ///
    /// ```
    /// use p2p_handshake_bitcoin::bitcoin::client::BitcoinClient;
    /// use p2p_handshake_bitcoin::bitcoin::stream::Stream;
    ///
    /// let ip_address_port = "127.0.0.1:8333";
    /// let timeout = 200; // In miliseconds
    /// async {
    ///     let stream = Stream::new(ip_address_port, 200).await.unwrap();
    ///     let bitcoin_client = BitcoinClient::new(stream.rx, stream.tx);
    /// };
    /// ```
    pub fn new(rx_stream: Reader, tx_stream: Writer) -> BitcoinClient<Reader, Writer> {
        let connection = Connection::new(rx_stream, tx_stream);
        BitcoinClient { connection }
    }

    /// Bitcoin client performs handshake with the remote node provided in
    /// It sends the version message, accepts the version message, sends back
    /// verack message and the accepts verack message.
    /// Use [Stream] module as the basis.
    /// Example shows localhost ip address, instead use real bitcoin node ip.
    ///
    /// [Stream]: crate::bitcoin::stream::Stream
    ///
    /// # Example
    ///
    /// ```
    /// use p2p_handshake_bitcoin::bitcoin::client::BitcoinClient;
    /// use p2p_handshake_bitcoin::bitcoin::stream::Stream;
    ///
    /// let ip_address_port = "127.0.0.1:8333";
    /// let timeout = 200; // In miliseconds
    /// async {
    ///     let stream = Stream::new(ip_address_port, 200).await.unwrap();
    ///     let mut bitcoin_client = BitcoinClient::new(stream.rx, stream.tx);
    ///     let result = bitcoin_client.handshake().await;
    /// };
    /// ```
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

    /// Bitcoin client with handle message sends message, receives response and
    /// checks whether there were any errors during the process.
    /// It sends the version message, accepts the version message, sends back
    /// verack message and the accepts verack message.
    /// Use [Stream] module as the basis and [VersionMessage] to send it to
    /// remote node.
    /// Example shows localhost as ip address, instead use real bitcoin node ip.
    ///
    /// [Stream]: crate::bitcoin::stream::Stream
    /// [VersionMessage]: crate::bitcoin::message::BitcoinMessage
    /// # Example
    ///
    /// ```
    /// use p2p_handshake_bitcoin::bitcoin::client::BitcoinClient;
    /// use p2p_handshake_bitcoin::bitcoin::stream::Stream;
    /// use p2p_handshake_bitcoin::bitcoin::message::BitcoinMessage;
    ///
    /// let ip_address_port = "127.0.0.1:8333";
    /// let timeout = 200; // In miliseconds
    /// async {
    ///     let stream = Stream::new(ip_address_port, 200).await.unwrap();
    ///     let mut bitcoin_client = BitcoinClient::new(stream.rx, stream.tx);
    ///     let result = bitcoin_client.handle_message(
    ///         BitcoinMessage::version_message()
    ///     ).await;
    /// };
    /// ```
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

    /// Basic version message verification
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

    /// Basic verack message verification
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
