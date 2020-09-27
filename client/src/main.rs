mod codec;
mod error;

use crate::codec::GameClientCodec;
use crate::error::ClientError;
use futures::{SinkExt, StreamExt};
use game_comms_messages::{ClientMessage, ServerMessage};
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let addr = "127.0.0.1:8000".to_string();
    let stream = TcpStream::connect(&addr).await?;

    let codec = GameClientCodec {};
    let (mut sink, mut input) = codec.framed(stream).split();

    sink.send(ClientMessage::HelloFromClient { client_version: 1 })
        .await?;

    while let Some(Ok(message)) = input.next().await {
        match message {
            ServerMessage::HelloFromServer { server_version } => {
                println!("Connected to server with version: {}", server_version);
            }

            ServerMessage::Ping => sink.send(ClientMessage::Pong).await?,
        }
    }

    Ok(())
}
