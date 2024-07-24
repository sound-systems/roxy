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

// use fastwebsockets::upgrade;
// use fastwebsockets::OpCode;
// use futures_util::{SinkExt, StreamExt};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};

// async fn handle_connection(
//     mut raw_stream: TcpStream,
//     addr: std::net::SocketAddr,
//     upstream_url: &str,
// ) {
//     // Perform WebSocket handshake
//     let (response, mut client_ws) = handshake::server(&mut raw_stream, None)
//         .await
//         .expect("Error during WebSocket handshake");

//     raw_stream
//         .write_all(response.as_bytes())
//         .await
//         .expect("Failed to send handshake response");
//     println!("New WebSocket connection from: {}", addr);

//     // Connect to upstream WebSocket server
//     let (mut upstream_raw_stream, _) = tokio::net::TcpStream::connect(upstream_url)
//         .await
//         .expect("Failed to connect to upstream WebSocket");

//     let request = format!(
//         "GET / HTTP/1.1\r\nHost: {}\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n\r\n",
//         upstream_url,
//         base64::encode("randomkey")
//     );

//     upstream_raw_stream
//         .write_all(request.as_bytes())
//         .await
//         .expect("Failed to send handshake request");

//     let mut response = vec![0; 1024];
//     let _ = upstream_raw_stream
//         .read(&mut response)
//         .await
//         .expect("Failed to read handshake response");

//     let mut upstream_ws = fastwebsockets::WebSocket::after_handshake(upstream_raw_stream)
//         .await
//         .expect("Error during WebSocket handshake with upstream");

//     println!("Connected to upstream WebSocket at: {}", upstream_url);

//     // Forward messages between client and upstream server
//     let (mut client_ws_sender, mut client_ws_receiver) = client_ws.split();
//     let (mut upstream_ws_sender, mut upstream_ws_receiver) = upstream_ws.split();

//     let client_to_upstream = async {
//         while let Some(Ok(msg)) = client_ws_receiver.next().await {
//             if msg.opcode == OpCode::Close {
//                 break;
//             }
//             upstream_ws_sender
//                 .send(msg)
//                 .await
//                 .expect("Failed to send message to upstream");
//         }
//     };

//     let upstream_to_client = async {
//         while let Some(Ok(msg)) = upstream_ws_receiver.next().await {
//             if msg.opcode == OpCode::Close {
//                 break;
//             }
//             client_ws_sender
//                 .send(msg)
//                 .await
//                 .expect("Failed to send message to client");
//         }
//     };

//     tokio::select! {
//         _ = client_to_upstream => println!("Client connection closed: {}", addr),
//         _ = upstream_to_client => println!("Upstream connection closed: {}", upstream_url),
//     }

//     println!("Connection handler finished for: {}", addr);
// }
