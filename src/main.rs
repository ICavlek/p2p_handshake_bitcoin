use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bitcoin::{
    consensus::{deserialize_partial, serialize},
    network::{
        address,
        constants::{Network, ServiceFlags},
        message::{NetworkMessage, RawNetworkMessage},
        message_network::VersionMessage,
    },
};

use bytes::BytesMut;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let dest_socket = "45.9.148.241:8333";
    let user_agent = "/Satoshi:23.0.0/";
    let timeout: u64 = 200;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let no_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let node_socket = SocketAddr::from_str(dest_socket).unwrap();

    let btc_version = VersionMessage::new(
        ServiceFlags::NONE,
        now,
        address::Address::new(&node_socket, ServiceFlags::NONE),
        address::Address::new(&no_address, ServiceFlags::NONE),
        now as u64,
        user_agent.to_string(),
        0,
    );

    let raw_message = RawNetworkMessage {
        magic: Network::Bitcoin.magic(),
        payload: NetworkMessage::Version(btc_version),
    };

    let stream = tokio::time::timeout(
        Duration::from_millis(timeout),
        TcpStream::connect(dest_socket),
    )
    .await
    .unwrap()
    .unwrap();

    let (rx_stream, mut tx_stream) = stream.into_split();

    let data = serialize(&raw_message);
    tx_stream.write_all(data.as_slice()).await.unwrap();

    let mut read_buffer = BytesMut::with_capacity(512);
    loop {
        if let Ok((message, count)) = deserialize_partial::<RawNetworkMessage>(&read_buffer) {
            println!("{}, {}", message.cmd(), count);
            break;
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
