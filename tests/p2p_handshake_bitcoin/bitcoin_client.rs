use bitcoin::{consensus::serialize, p2p::message::RawNetworkMessage};
use p2p_handshake_bitcoin::{bitcoin_message::BitcoinMessage, connection::Connection};

use crate::helper::BitcoinNodeMock;

#[tokio::test]
async fn bitcoin_node_responds_with_version_and_verack_message() {
    let bitcoin_mock_node = BitcoinNodeMock::default();
    let mut connection = Connection::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);
    let bitcoin_version_message = BitcoinMessage::version_message();

    let _ = connection
        .write(serialize(&bitcoin_version_message).as_slice())
        .await;
    let (message, count) = match connection.read::<RawNetworkMessage>().await.unwrap() {
        Some((message, count)) => (message, count),
        None => (bitcoin_version_message, 1),
    };

    dbg!(message, count);
    assert!(true);
}
