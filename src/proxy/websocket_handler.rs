use std::{net::SocketAddr, pin::Pin};

use anyhow::{Context, Error};
use fastwebsockets::{
    handshake,
    upgrade::{upgrade, UpgradeFut},
    OpCode, Role, WebSocket, WebSocketError,
};

use http_body_util::Empty;
use hyper::{
    body::{Bytes, Incoming},
    header::{CONNECTION, UPGRADE},
    service::Service,
    Request, Response,
};
use std::future::Future;

use tokio::net::TcpStream;

use super::uri_determinator::Registry;

// Tie hyper's executor to tokio runtime
struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}

async fn handle_connection(client_ws: UpgradeFut, addr: SocketAddr) -> Result<(), Error> {
    let client_ws = client_ws.await?;

    let stream = TcpStream::connect(addr)
        .await
        .context("failed to connect to upstream server")?;

    let req = Request::builder()
        .method("GET")
        .uri("http://localhost:9001/")
        .header("Host", "localhost:9001")
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header("Sec-WebSocket-Key", handshake::generate_key())
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (upstream_ws, _) = handshake::client(&SpawnExecutor, req, stream)
        .await
        .context("failed to establish websocket upgrade with upstream")?;

    let (mut client_recv, mut client_write) = client_ws.split(tokio::io::split);
    let (mut upstream_recv, mut upstream_write) = upstream_ws.split(tokio::io::split);

    let client_to_upstream = async {
        while let Ok(frame) = client_recv
            .read_frame::<_, WebSocketError>(&mut move |_| async {
                unreachable!();
            })
            .await
        {
            upstream_write
                .write_frame(frame)
                .await
                .expect("failed to write to upstream");
        }
    };

    let upstream_to_client = async {
        while let Ok(frame) = upstream_recv
            .read_frame::<_, WebSocketError>(&mut move |_| async {
                unreachable!();
            })
            .await
        {
            client_write
                .write_frame(frame)
                .await
                .expect("failed to write to upstream");
        }
    };

    tokio::select! {
        _ = client_to_upstream => println!("Client connection closed"),
        _ = upstream_to_client => println!("Upstream connection closed: {}", addr),
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct WebSocketHandler {
    registry: Registry,
}

impl Service<Request<Incoming>> for WebSocketHandler {
    type Response = Response<Empty<Bytes>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<Incoming>) -> Self::Future {
        let res = match upgrade(&mut req) {
            Ok((res, ws)) => {
                let registry = self.registry.clone();
                tokio::spawn(async move {
                    if let Err(e) = match registry.determine_address().await {
                        Ok(uri) => handle_connection(ws, uri)
                            .await
                            .context("websocket proxy connection broke"),
                        Err(e) => Err(e).context("failed to determine upstream uri"),
                    } {
                        eprint!("something went wrong: {e}")
                    }
                });
                Ok(res)
            }
            Err(e) => Err(e).context("websocket handshake failed: could not upgrade connection"),
        };

        Box::pin(async { res })
    }
}
