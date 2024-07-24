pub mod config;
mod proxy;

use std::net::SocketAddr;

use anyhow::Error;
pub use config::Settings;

pub async fn start(config: Settings) -> Result<(), Error> {
    let registry = proxy::Registry::new(vec!["127.0.0.1:8081", "127.0.0.1:8082", "127.0.0.1:8083"]);
    let handler = proxy::WebSocketHandler::new(registry);
    let proxy = proxy::Server::new(handler);
    let addr: SocketAddr = format!("{}:{}", config.web.host, config.web.port).parse()?;

    proxy.listen(addr).await?;

    Ok(())
}
