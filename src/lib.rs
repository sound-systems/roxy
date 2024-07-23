pub mod config;
mod proxy;

use anyhow::Error;
pub use config::Settings;

pub async fn start(config: Settings) -> Result<(), Error> {
    Ok(())
}
