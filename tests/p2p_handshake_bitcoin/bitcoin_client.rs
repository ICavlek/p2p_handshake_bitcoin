use p2p_handshake_bitcoin::{bitcoin_client::BitcoinClient, bitcoin_message::BitcoinMessage};

use crate::helper::BitcoinNodeMock;

#[tokio::test]
async fn bitcoin_node_responds_with_version_and_verack_message() {
    let bitcoin_mock_node = BitcoinNodeMock::default();
    let mut bitcoin_client = BitcoinClient::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer)
        .expect("Failed to create Bitcoin client");

    let bitcoin_version_message = BitcoinMessage::version_message();
    bitcoin_client
        .handle_message(bitcoin_version_message)
        .await
        .expect("Failed to exchange version messages");
    let bitcoin_verack_message = BitcoinMessage::verack_message();
    bitcoin_client
        .handle_message(bitcoin_verack_message)
        .await
        .expect("Failed to exchange verack messages");
}
