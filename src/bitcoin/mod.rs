/// Client that is used to establish communication with the remote node
pub mod client;
/// Module to handle multiple bitcoin client handshakes
pub mod client_pool;
/// Module that handles connection and message exchange with Bitcoin node
pub mod connection;
pub mod message;
pub mod stream;
