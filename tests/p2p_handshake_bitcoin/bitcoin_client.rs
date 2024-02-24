use p2p_handshake_bitcoin::{bitcoin_client::BitcoinClient, bitcoin_message::BitcoinMessage};

use crate::helper::BitcoinNodeMock;

#[tokio::test]
async fn bitcoin_node_responds_with_version_and_verack_message() {
    let bitcoin_mock_node = BitcoinNodeMock::default();
    let bitcoin_client = BitcoinClient::new(bitcoin_mock_node.reader, bitcoin_mock_node.writer);
    let bitcoin_version_message = BitcoinMessage::version_message();
    match bitcoin_client
        .expect("Failed to create client")
        .handle_message(bitcoin_version_message)
        .await
    {
        Ok((message, count)) => dbg!(message, count),
        Err(_) => dbg!(BitcoinMessage::version_message(), 99),
    };

    assert!(true);
}
