//! # proxy
//!
//! the proxy module contains the utilities to
//! bootstrap a websocket proxy
//!

mod connection;
mod connection_handler;
mod server;
mod uri_determinator;
mod websocket_handler;

pub use server::Server;
pub use uri_determinator::Registry;
pub use websocket_handler::WebSocketHandler;
