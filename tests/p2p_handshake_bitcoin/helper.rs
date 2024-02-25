use bitcoin::{
    consensus::serialize,
    p2p::message::{NetworkMessage, RawNetworkMessage},
    Network,
};
use p2p_handshake_bitcoin::bitcoin_message::BitcoinMessage;
use tokio_test::io::{Builder, Mock};

pub struct BitcoinNodeMock {
    pub reader: Mock,
    pub writer: Mock,
}

impl Default for BitcoinNodeMock {
    fn default() -> Self {
        let bitcoin_version_message = BitcoinMessage::version_message();
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        Self {
            reader: Builder::new()
                .read(serialize(&bitcoin_version_message).as_slice())
                .read(serialize(&bitcoin_verack_message).as_slice())
                .build(),
            writer: Builder::new()
                .write(serialize(&bitcoin_version_message).as_slice())
                .write(serialize(&bitcoin_verack_message).as_slice())
                .build(),
        }
    }
}

impl BitcoinNodeMock {
    pub fn bad_u8_slice_response_on_version_message() -> BitcoinNodeMock {
        let bitcoin_version_message = BitcoinMessage::version_message();
        Self {
            reader: Builder::new().read(&[1, 2, 3]).build(),
            writer: Builder::new()
                .write(serialize(&bitcoin_version_message).as_slice())
                .build(),
        }
    }

    pub fn on_version_message_respond_with_verack_message() -> BitcoinNodeMock {
        let bitcoin_version_message = BitcoinMessage::version_message();
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        Self {
            reader: Builder::new()
                .read(serialize(&bitcoin_verack_message).as_slice())
                .build(),
            writer: Builder::new()
                .write(serialize(&bitcoin_version_message).as_slice())
                .build(),
        }
    }

    pub fn on_version_message_respond_with_malicious_version_message() -> BitcoinNodeMock {
        let bitcoin_version_message = BitcoinMessage::version_message();
        let bitcoin_wrong_version_message = BitcoinWrongMessage::wrong_version_message();
        Self {
            reader: Builder::new()
                .read(serialize(&bitcoin_wrong_version_message).as_slice())
                .build(),
            writer: Builder::new()
                .write(serialize(&bitcoin_version_message).as_slice())
                .build(),
        }
    }

    pub fn on_verack_message_responds_with_version_message() -> BitcoinNodeMock {
        let bitcoin_version_message = BitcoinMessage::version_message();
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        Self {
            reader: Builder::new()
                .read(serialize(&bitcoin_version_message).as_slice())
                .read(serialize(&bitcoin_version_message).as_slice())
                .build(),
            writer: Builder::new()
                .write(serialize(&bitcoin_version_message).as_slice())
                .write(serialize(&bitcoin_verack_message).as_slice())
                .build(),
        }
    }
}

pub struct BitcoinWrongMessage;

impl BitcoinWrongMessage {
    pub fn wrong_version_message() -> RawNetworkMessage {
        let mut bitcoin_version_message = BitcoinMessage::get_bitcoin_version_message();
        bitcoin_version_message.version = 99;
        RawNetworkMessage::new(
            Network::Bitcoin.magic(),
            NetworkMessage::Version(bitcoin_version_message),
        )
    }
}
