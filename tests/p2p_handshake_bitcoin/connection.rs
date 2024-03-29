use bitcoin::{consensus::serialize, p2p::message::RawNetworkMessage};
use p2p_handshake_bitcoin::{bitcoin::connection::Connection, bitcoin::message::BitcoinMessage};

use crate::helper::{BitcoinNodeMock, BitcoinWrongMessage};

#[tokio::test]
async fn bitcoin_node_responds_with_version_and_verack_message() {
    let bitcoin_mock_node = BitcoinNodeMock::default();
    let mut connection = Connection::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);

    let bitcoin_version_message = BitcoinMessage::version_message();
    connection
        .write(serialize(&bitcoin_version_message).as_slice())
        .await
        .expect("Failed to exchange version messages");
    let (message, count) = match connection.read::<RawNetworkMessage>().await.unwrap() {
        Some((message, count)) => (message, count),
        None => (BitcoinMessage::verack_message(), 99),
    };
    assert_eq!(message, BitcoinMessage::version_message());
    assert_eq!(count, 126);

    let bitcoin_verack_message = BitcoinMessage::verack_message();
    connection
        .write(serialize(&bitcoin_verack_message).as_slice())
        .await
        .expect("Failed to exchange verack messages");
    let (message, count) = match connection.read::<RawNetworkMessage>().await.unwrap() {
        Some((message, count)) => (message, count),
        None => (BitcoinMessage::version_message(), 99),
    };
    assert_eq!(message, BitcoinMessage::verack_message());
    assert_eq!(count, 24);
}

#[tokio::test]
async fn bitcoin_node_responds_with_bad_u8_slice() {
    let bitcoin_mock_node = BitcoinNodeMock::bad_u8_slice_response_on_version_message();
    let mut connection = Connection::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);

    let bitcoin_version_message = BitcoinMessage::version_message();
    let _ = connection
        .write(serialize(&bitcoin_version_message).as_slice())
        .await;
    let response = connection.read::<RawNetworkMessage>().await;
    assert!(response.is_err());
}

#[tokio::test]
async fn bitcoin_node_responds_on_version_message_with_verack_message() {
    let bitcoin_mock_node = BitcoinNodeMock::on_version_message_respond_with_verack_message();
    let mut connection = Connection::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);

    let bitcoin_version_message = BitcoinMessage::version_message();
    let _ = connection
        .write(serialize(&bitcoin_version_message).as_slice())
        .await;
    let (message, _) = match connection.read::<RawNetworkMessage>().await.unwrap() {
        Some((message, count)) => (message, count),
        None => (BitcoinMessage::version_message(), 99),
    };
    assert_eq!(message, BitcoinMessage::verack_message());
}

#[tokio::test]
async fn bitcoin_node_responds_with_malicious_version() {
    let bitcoin_node_mock =
        BitcoinNodeMock::on_version_message_respond_with_malicious_version_message();
    let mut connection = Connection::new(bitcoin_node_mock.reader, bitcoin_node_mock.writer);

    let bitcoin_version_message = BitcoinMessage::version_message();
    let _ = connection
        .write(serialize(&bitcoin_version_message).as_slice())
        .await;
    let (message, _) = match connection.read::<RawNetworkMessage>().await.unwrap() {
        Some((message, count)) => (message, count),
        None => (BitcoinMessage::version_message(), 99),
    };
    assert_eq!(message, BitcoinWrongMessage::wrong_version_message());
}
