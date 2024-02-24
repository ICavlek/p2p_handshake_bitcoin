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
    Reader: AsyncReadExt + Unpin,
    Writer: AsyncWriteExt + Unpin,
{
    pub fn new(
        rx_stream: Reader,
        tx_stream: Writer,
    ) -> Result<BitcoinClient<Reader, Writer>, anyhow::Error> {
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
