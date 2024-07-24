use anyhow::Error;
use fastwebsockets::{
    handshake,
    upgrade::{self, upgrade},
    WebSocket,
};
use tokio::net::TcpStream;

pub async fn handle(stream: TcpStream) -> Result<(), Error> {
    Ok(())
}
