use bytes::{Buf, BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed, LinesCodecError};

use crate::ServerMessage::Hello;
use futures::io::Error;
use futures::stream::{SplitSink, SplitStream};
use std::sync::{Arc, Mutex};

struct Shared;

impl Shared {
    fn new() -> Self {
        Self {}
    }
}

struct GameCodec;

impl GameCodec {
    fn new() -> Self {
        Self {}
    }
}

struct ClientHelloMessage {
    version: u32,
}

enum ClientMessage {
    Hello(ClientHelloMessage),
}

impl Into<u32> for ClientMessage {
    fn into(self) -> u32 {
        match self {
            ClientMessage::Hello(_m) => 1_u32,
        }
    }
}

enum ServerMessage {
    Hello,
}

#[derive(Debug)]
enum ServerError {
    InvalidSomething,
    LinesCodecError(LinesCodecError),
    IO(std::io::Error),
    ParseError(tracing_subscriber::filter::ParseError),
}

impl From<std::io::Error> for ServerError {
    fn from(error: Error) -> Self {
        ServerError::IO(error)
    }
}

impl From<LinesCodecError> for ServerError {
    fn from(error: LinesCodecError) -> Self {
        ServerError::LinesCodecError(error)
    }
}

impl From<tracing_subscriber::filter::ParseError> for ServerError {
    fn from(error: tracing_subscriber::filter::ParseError) -> Self {
        ServerError::ParseError(error)
    }
}

impl tokio_util::codec::Decoder for GameCodec {
    type Item = ClientMessage;
    type Error = ServerError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            return Ok(None);
        }

        // Read the message type from the buffer.
        let message_type = src.get_u32();

        let message = match message_type {
            1_u32 => ClientMessage::Hello(ClientHelloMessage {
                version: src.get_u32(),
            }),
            _ => return Err(ServerError::InvalidSomething),
        };

        Ok(Some(message))
    }
}

impl tokio_util::codec::Encoder<ServerMessage> for GameCodec {
    type Error = ServerError;

    fn encode(&mut self, item: ServerMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Hello => {
                dst.put_u32(1);
            }
        }

        Ok(())
    }
}

struct Client {
    _shared: Arc<Mutex<Shared>>,
    sink: SplitSink<Framed<TcpStream, GameCodec>, ServerMessage>,
    input: SplitStream<Framed<TcpStream, GameCodec>>,
}

impl Client {
    fn new(shared: Arc<Mutex<Shared>>, stream: TcpStream) -> Self {
        let codec = GameCodec::new();
        let (sink, input) = codec.framed(stream).split();
        Self {
            _shared: shared,
            sink,
            input,
        }
    }

    async fn run(&mut self) -> Result<(), ServerError> {
        while let Some(Ok(message)) = self.input.next().await {
            self.handle_client_message(message).await?
        }

        Ok(())
    }

    async fn handle_client_message(&mut self, message: ClientMessage) -> Result<(), ServerError> {
        match message {
            ClientMessage::Hello(message) => {
                println!("Client connected with version: {}", message.version);
                self.send_hello().await?;
            }
        }

        Ok(())
    }

    async fn send_hello(&mut self) -> Result<(), ServerError> {
        self.sink.send(ServerMessage::Hello).await
    }
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("server=info".parse()?))
        .with_span_events(FmtSpan::FULL)
        .init();

    // Create the shared state between all clients.
    let shared = Arc::new(Mutex::new(Shared::new()));

    let addr = "127.0.0.1:8000".to_string();
    let mut listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening for connections on tcp://{}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;

        tracing::debug!("Accepted connection from: {}", &addr);

        let client_shared = Arc::clone(&shared);
        let mut client = Client::new(client_shared, stream);

        tokio::spawn(async move {
            if let Err(err) = client.run().await {
                return Err(err);
            };

            Ok(())
        });
    }
}
