use bitcoin::consensus::{deserialize_partial, Decodable};
use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Connection<Reader, Writer>
where
    Reader: AsyncReadExt + Unpin,
    Writer: AsyncWriteExt + Unpin,
{
    rx_stream: Reader,
    tx_stream: Writer,
    buffer: BytesMut,
}

impl<Reader, Writer> Connection<Reader, Writer>
where
    Reader: AsyncReadExt + Unpin,
    Writer: AsyncWriteExt + Unpin,
{
    pub fn new(rx_stream: Reader, tx_stream: Writer) -> Connection<Reader, Writer>
    where
        Reader: AsyncReadExt + Unpin,
        Writer: AsyncWriteExt + Unpin,
    {
        Connection {
            rx_stream,
            tx_stream,
            buffer: BytesMut::with_capacity(2048),
        }
    }

    pub async fn read<T: Decodable>(&mut self) -> Result<Option<(T, usize)>, anyhow::Error> {
        loop {
            if let Ok((message, count)) = deserialize_partial::<T>(&self.buffer) {
                self.buffer.advance(count);
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

    pub async fn write(&mut self, message: &[u8]) -> Result<(), anyhow::Error> {
        self.tx_stream.write_all(message).await?;
        Ok(())
    }
}
