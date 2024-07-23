use std::error::Error;

use clap::Parser;
use roxy::{config::Arguments, Settings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    let settings: Settings = args.config.try_into()?;
    roxy::start(settings).await?;
    Ok(())
}
