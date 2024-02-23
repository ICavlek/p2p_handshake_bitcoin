use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};

use bitcoin::{
    p2p::{
        address,
        message::{NetworkMessage, RawNetworkMessage},
        message_network::VersionMessage,
        ServiceFlags,
    },
    Network,
};

pub struct BitcoinMessage;

impl BitcoinMessage {
    pub fn default() -> RawNetworkMessage {
        let user_agent = "/Satoshi:26.0.0/";

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let no_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let node_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

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
}
