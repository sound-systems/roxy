use std::net::SocketAddr;

use anyhow::{Context, Error};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use super::websocket_handler::WebSocketHandler;

pub struct Server {
    handler: WebSocketHandler,
}

impl Server {
    pub fn new(handler: WebSocketHandler) -> Self {
        Self { handler }
    }

    pub async fn listen(self, addr: SocketAddr) -> Result<(), Error> {
        let listener = TcpListener::bind(addr)
            .await
            .context("failed to setup proxy tcp listener")?;

        while let Ok((stream, _addr)) = listener.accept().await {
            let io = TokioIo::new(stream);
            let handler = self.handler.clone();
            tokio::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, handler)
                    .with_upgrades()
                    .await
                {
                    eprintln!("error serving websocket connection: {:?}", err);
                }
            });
        }

        Ok(())
    }
}
