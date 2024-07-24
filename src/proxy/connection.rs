use anyhow::Error;
use tokio::net::TcpStream;

pub async fn handle(stream: TcpStream) -> Result<(), Error> {
    Ok(())
}
