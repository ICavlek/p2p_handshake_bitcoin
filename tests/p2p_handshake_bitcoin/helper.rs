use bitcoin::consensus::serialize;
use p2p_handshake_bitcoin::bitcoin_message::BitcoinMessage;
use tokio_test::io::Mock;

pub struct BitcoinNodeMock {
    pub reader: Mock,
    pub writer: Mock,
}

impl Default for BitcoinNodeMock {
    fn default() -> Self {
        let bitcoin_version_message = BitcoinMessage::version_message();
        let bitcoin_verack_message = BitcoinMessage::verack_message();
        Self {
            reader: tokio_test::io::Builder::new()
                .read(serialize(&bitcoin_version_message).as_slice())
                .read(serialize(&bitcoin_verack_message).as_slice())
                .build(),
            writer: tokio_test::io::Builder::new()
                .write(serialize(&bitcoin_version_message).as_slice())
                .write(serialize(&bitcoin_verack_message).as_slice())
                .build(),
        }
    }
}
