use std::{io, time::Duration};

use bitcoin::{
    consensus::{deserialize_partial, serialize},
    p2p::message::RawNetworkMessage,
};
use bytes::BytesMut;
use tokio::{
    io::AsyncWriteExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
};

pub struct Client {
    pub connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, anyhow::Error> {
        let socket = tokio::time::timeout(Duration::from_millis(500), TcpStream::connect(addr))
            .await
            .unwrap()
            .unwrap();
        let (rx_stream, tx_stream) = socket.into_split();
        let connection = Connection::new(rx_stream, tx_stream);
        Ok(Client { connection })
    }
}

#[derive(Debug)]
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
            buffer: BytesMut::with_capacity(2048),
        }
    }

    pub fn read(&mut self) -> Result<(RawNetworkMessage, usize), anyhow::Error> {
        loop {
            if let Ok((message, count)) = deserialize_partial::<RawNetworkMessage>(&self.buffer) {
                return Ok((message, count));
            }
            match self.rx_stream.try_read_buf(&mut self.buffer) {
                Ok(value) => {
                    if value == 0 {
                        if self.buffer.is_empty() {
                            println!("Empty buffer");
                        } else {
                            println!("Connection error");
                        }
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    pub async fn write(&mut self, message: RawNetworkMessage) -> io::Result<()> {
        self.tx_stream
            .write_all(serialize(&message).as_slice())
            .await?;
        Ok(())
    }
}
