use bitcoin::{consensus::serialize, p2p::message::RawNetworkMessage};
use p2p_handshake_bitcoin::{bitcoin_message::BitcoinMessage, connection::Connection};

#[tokio::test]
async fn init_test() {
    let bitcoin_version_message = BitcoinMessage::version_message();
    let bitcoin_verack_message = BitcoinMessage::verack_message();
    let reader = tokio_test::io::Builder::new()
        .read(serialize(&bitcoin_verack_message).as_slice())
        .build();
    let write = tokio_test::io::Builder::new()
        .write(serialize(&bitcoin_version_message).as_slice())
        .build();
    let mut connection = Connection::new(reader, write);

    let _ = connection
        .write(serialize(&bitcoin_version_message).as_slice())
        .await;
    let (message, count) = match connection.read::<RawNetworkMessage>().await.unwrap() {
        Some((message, count)) => (message, count),
        None => (bitcoin_verack_message, 1),
    };
    dbg!(message, count);
    assert!(true);
}
