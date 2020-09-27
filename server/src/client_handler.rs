use crate::codec::GameCodec;
use crate::error::ServerError;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use game_comms_messages::{ClientMessage, ServerMessage};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed};

pub struct ClientHandler {
    sink: SplitSink<Framed<TcpStream, GameCodec>, ServerMessage>,
    input: SplitStream<Framed<TcpStream, GameCodec>>,
}

impl ClientHandler {
    pub fn new(stream: TcpStream) -> Self {
        let codec = GameCodec::new();
        let (sink, input) = codec.framed(stream).split();
        Self { sink, input }
    }

    pub async fn run(&mut self) -> Result<(), ServerError> {
        // tokio::spawn(async {
        //     tokio::time::delay_for(Duration::from_secs(1)).await;
        //     self.send_ping().await;
        // });

        while let Some(Ok(message)) = self.input.next().await {
            self.handle_client_message(message).await?
        }

        Ok(())
    }

    async fn handle_client_message(&mut self, message: ClientMessage) -> Result<(), ServerError> {
        match message {
            ClientMessage::HelloFromClient { client_version } => {
                println!("Client connected with version: {}", client_version);
                self.send_hello().await?;
            }

            ClientMessage::Pong => {
                println!("pong");
            }
        }

        Ok(())
    }

    async fn send_hello(&mut self) -> Result<(), ServerError> {
        self.sink
            .send(ServerMessage::HelloFromServer { server_version: 1 })
            .await?;
        Ok(())
    }

    // async fn send_ping(&mut self) -> Result<(), ServerError> {
    //     self.sink.send(ServerMessage::Ping).await?;
    //     Ok(())
    // }
}
