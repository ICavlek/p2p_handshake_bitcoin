/// Client that is used to establish communication with the remote node
pub mod client;
/// Module to handle multiple bitcoin client handshakes
pub mod client_pool;
/// Module that handles connection and message exchange with Bitcoin node
pub mod connection;
/// Module that creates Bitcoin compatible messages
pub mod message;
/// Module that provides reading and writing streams
pub mod stream;
