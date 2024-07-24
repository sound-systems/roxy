use fastwebsockets::upgrade;
use fastwebsockets::FragmentCollectorRead;
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper::Response;
use tokio::net::TcpListener;

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let ws = fut.await?;
    let (rx, mut tx) = ws.split(tokio::io::split);
    let mut rx = FragmentCollectorRead::new(rx);
    loop {
        // Empty send_fn is fine because the benchmark does not create obligated writes.
        let frame = rx
            .read_frame::<_, WebSocketError>(&mut move |_| async {
                unreachable!();
            })
            .await?;
        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
                tx.write_frame(frame).await?;
            }
            _ => {}
        }
    }

    Ok(())
}
async fn server_upgrade(
    mut req: Request<Incoming>,
) -> Result<Response<Empty<Bytes>>, WebSocketError> {
    let (response, fut) = upgrade::upgrade(&mut req)?;

    tokio::task::spawn(async move {
        if let Err(e) = tokio::task::unconstrained(handle_client(fut)).await {
            eprintln!("Error in websocket connection: {}", e);
        }
    });

    Ok(response)
}
