use std::net::SocketAddr;

use anyhow::{Context, Error};
use tokio::net::TcpListener;

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn listen(self) -> Result<(), Error> {
        TcpListener::bind(self.addr)
            .await
            .context("failed to setup proxy tcp listener")?;
        Ok(())
    }
}
