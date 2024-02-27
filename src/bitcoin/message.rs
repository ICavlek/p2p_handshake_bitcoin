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

/// Module that creates Bitcoin compatible messages
pub struct BitcoinMessage;

impl BitcoinMessage {
    /// Returns a VersionMessage
    pub fn get_bitcoin_version_message() -> VersionMessage {
        let user_agent = "/Satoshi:26.0.0/";

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let receiver_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let sender_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

        let receiver_address = address::Address::new(&receiver_socket, ServiceFlags::NONE);
        let sender_address = address::Address::new(&sender_socket, ServiceFlags::NONE);

        VersionMessage::new(
            ServiceFlags::NONE,
            now,
            receiver_address,
            sender_address,
            now as u64,
            user_agent.to_string(),
            0,
        )
    }

    /// Returns a VersionMessage which can be sent to Bitcoin node
    pub fn version_message() -> RawNetworkMessage {
        RawNetworkMessage::new(
            Network::Bitcoin.magic(),
            NetworkMessage::Version(BitcoinMessage::get_bitcoin_version_message()),
        )
    }

    /// Returns a VerackMessage which can be sent to Bitcoin node
    pub fn verack_message() -> RawNetworkMessage {
        RawNetworkMessage::new(Network::Bitcoin.magic(), NetworkMessage::Verack)
    }
}
