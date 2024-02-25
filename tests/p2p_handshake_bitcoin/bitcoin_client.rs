use p2p_handshake_bitcoin::{
    bitcoin_client::{BitcoinClient, BitcoinClientError},
    bitcoin_message::BitcoinMessage,
};

use crate::helper::BitcoinNodeMock;

#[tokio::test]
async fn bitcoin_node_responds_with_version_and_verack_message() {
    let bitcoin_mock_node = BitcoinNodeMock::default();
    let mut bitcoin_client = BitcoinClient::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);

    let bitcoin_version_message = BitcoinMessage::version_message();
    let (message, count) = bitcoin_client
        .handle_message(bitcoin_version_message)
        .await
        .expect("Failed to exchange version messages");
    assert_eq!(message, BitcoinMessage::version_message());
    assert_eq!(count, 126);

    let bitcoin_verack_message = BitcoinMessage::verack_message();
    let (message, count) = bitcoin_client
        .handle_message(bitcoin_verack_message)
        .await
        .expect("Failed to exchange verack messages");
    assert_eq!(message, BitcoinMessage::verack_message());
    assert_eq!(count, 24);
}

#[tokio::test]
async fn bitcoin_node_responds_with_bad_u8_slice() {
    let bitcoin_mock_node = BitcoinNodeMock::bad_u8_slice_response_on_version_message();
    let mut bitcoin_client = BitcoinClient::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);
    let response = bitcoin_client.handshake().await;
    assert!(matches!(
        response,
        Err(BitcoinClientError::CommunicationError)
    ));
}

#[tokio::test]
async fn bitcoin_node_responds_with_verack_message_on_version_message() {
    let bitcoin_mock_node = BitcoinNodeMock::on_version_message_respond_with_verack_message();
    let mut bitcoin_client = BitcoinClient::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);
    let response = bitcoin_client.handshake().await;
    assert!(response.is_err());
}

#[tokio::test]
async fn bitcoin_node_responds_with_malicious_version() {
    let bitcoin_node_mock =
        BitcoinNodeMock::on_version_message_respond_with_malicious_version_message();
    let mut bitcoin_client = BitcoinClient::new(bitcoin_node_mock.reader, bitcoin_node_mock.writer);
    let response = bitcoin_client.handshake().await;
    assert!(matches!(response, Err(BitcoinClientError::MessageError)));
}

#[tokio::test]
async fn bitcoin_node_responds_with_version_message_on_verack_message() {
    let bitcoin_node_mock = BitcoinNodeMock::on_verack_message_responds_with_version_message();
    let mut bitcoin_client = BitcoinClient::new(bitcoin_node_mock.reader, bitcoin_node_mock.writer);
    let response = bitcoin_client.handshake().await;
    assert!(matches!(response, Err(BitcoinClientError::MessageError)));
}
