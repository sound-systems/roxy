use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub web: WebServer,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            web: Default::default(),
        }
    }
}

impl TryFrom<Option<PathBuf>> for Settings {
    type Error = anyhow::Error;

    fn try_from(value: Option<PathBuf>) -> Result<Self, Self::Error> {
        match value {
            Some(path) => {
                let config_file =
                    fs::read_to_string(path).context("could not open configuration file")?;
                let settings: Settings = toml::from_str(&config_file)
                    .context("provided config file is not valid toml")?;
                Ok(settings)
            }
            None => Ok(Default::default()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WebServer {
    pub host: String,
    pub port: u16,
}

impl Default for WebServer {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 9090,
        }
    }
}
