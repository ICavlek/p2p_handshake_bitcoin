use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bitcoin::{
    consensus::{deserialize_partial, serialize},
    network::Network,
    p2p::{
        address,
        message::{NetworkMessage, RawNetworkMessage},
        message_network::VersionMessage,
        ServiceFlags,
    },
};

use bytes::BytesMut;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct BitcoinClient {
    uri: String,
    timeout: u64,
}

impl BitcoinClient {
    pub fn new(uri: String, timeout: u64) -> Self {
        Self { uri, timeout }
    }

    pub async fn handshake(&self) {
        let raw_message = self.get_default_version_message();
        let (response, count) = self.send_message(raw_message).await;
        println!("{:#?}, {}", response, count);
    }

    fn get_default_version_message(&self) -> RawNetworkMessage {
        let user_agent = "/Satoshi:26.0.0/";

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let no_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let node_socket = SocketAddr::from_str(&self.uri).unwrap();

        let dest_address = address::Address::new(&node_socket, ServiceFlags::NONE);
        let source_address = address::Address::new(&no_address, ServiceFlags::NONE);

        let btc_version = VersionMessage::new(
            ServiceFlags::NONE,
            now,
            dest_address,
            source_address,
            now as u64,
            user_agent.to_string(),
            0,
        );

        RawNetworkMessage::new(
            Network::Bitcoin.magic(),
            NetworkMessage::Version(btc_version),
        )
    }

    async fn send_message(&self, message: RawNetworkMessage) -> (RawNetworkMessage, usize) {
        let stream = tokio::time::timeout(
            Duration::from_millis(self.timeout),
            TcpStream::connect(&self.uri),
        )
        .await
        .unwrap()
        .unwrap();

        let (rx_stream, mut tx_stream) = stream.into_split();

        let data = serialize(&message);
        tx_stream.write_all(data.as_slice()).await.unwrap();

        let mut read_buffer = BytesMut::with_capacity(512);
        loop {
            if let Ok((message, count)) = deserialize_partial::<RawNetworkMessage>(&read_buffer) {
                return (message, count);
            }
            match rx_stream.try_read_buf(&mut read_buffer) {
                Ok(value) => {
                    if value == 0 {
                        if read_buffer.is_empty() {
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
}
