use bytes::{Buf, BufMut, BytesMut};
use futures::io::Error;
use game_comms_messages::{ClientMessage, ServerMessage};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub enum CodecError {
    InvalidMessageType(u32),
    IO(std::io::Error),
}

impl From<std::io::Error> for CodecError {
    fn from(err: Error) -> Self {
        CodecError::IO(err)
    }
}

pub struct GameCodec;

impl GameCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for GameCodec {
    type Item = ClientMessage;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            return Ok(None);
        }

        // Read the message type from the buffer.
        let message_type = src.get_u32();

        let message = match message_type {
            1_u32 => ClientMessage::HelloFromClient {
                client_version: src.get_u32(),
            },
            _ => return Err(CodecError::InvalidMessageType(message_type)),
        };

        Ok(Some(message))
    }
}

impl Encoder<ServerMessage> for GameCodec {
    type Error = CodecError;

    fn encode(&mut self, item: ServerMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            ServerMessage::HelloFromServer { server_version } => {
                dst.put_u32(1);
                dst.put_u32(server_version);
            }

            ServerMessage::Ping => {
                dst.put_u32(2);
            }
        }

        Ok(())
    }
}
