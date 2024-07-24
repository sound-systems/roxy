use std::{net::SocketAddr, str::FromStr};

use anyhow::{Context, Error};
use hyper::Uri;

#[derive(Debug, Clone)]
pub struct Registry();

impl Registry {
    pub fn new() -> Self {
        Self()
    }

    pub async fn determine_address(&self) -> Result<SocketAddr, Error> {
        SocketAddr::from_str("http://localhost:6060/ws")
            .context("the registry has resolved to an invalid uri")
    }
}
