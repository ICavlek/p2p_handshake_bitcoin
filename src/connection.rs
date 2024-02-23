use bitcoin::{
    consensus::{deserialize_partial, serialize},
    p2p::message::RawNetworkMessage,
};
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub struct Connection {
    rx_stream: OwnedReadHalf,
    tx_stream: OwnedWriteHalf,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(rx_stream: OwnedReadHalf, tx_stream: OwnedWriteHalf) -> Connection {
        Connection {
            rx_stream,
            tx_stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub async fn read(&mut self) -> Result<Option<(RawNetworkMessage, usize)>, anyhow::Error> {
        loop {
            if let Ok((message, count)) = deserialize_partial::<RawNetworkMessage>(&self.buffer) {
                return Ok(Some((message, count)));
            }

            if self.rx_stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(anyhow::anyhow!("connection reset by peer"));
                }
            }
        }
    }

    pub async fn write(&mut self, message: RawNetworkMessage) -> Result<(), anyhow::Error> {
        self.tx_stream
            .write_all(serialize(&message).as_slice())
            .await?;
        Ok(())
    }
}
