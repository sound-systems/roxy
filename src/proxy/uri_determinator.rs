use std::{net::SocketAddr, str::FromStr};

use anyhow::{Context, Error};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Registry {
    addresses: Vec<String>,
}

impl Registry {
    pub fn new(addrs: Vec<&str>) -> Self {
        Self {
            addresses: addrs.into_iter().map(|a| a.to_string()).collect(),
        }
    }

    pub async fn determine_address(&self) -> Result<SocketAddr, Error> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.addresses.len());
        let addr = self
            .addresses
            .get(index)
            .context("random load balancer accessed invalid index")?;
        SocketAddr::from_str(addr).context("the registry has resolved to an invalid uri")
    }
}
