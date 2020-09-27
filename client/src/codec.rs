use bytes::{Buf, BufMut, BytesMut};
use futures::io::Error;
use game_comms_messages::{ClientMessage, ServerMessage};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub enum CodecError {
    InvalidMessageType,
    IO(std::io::Error),
}

impl From<std::io::Error> for CodecError {
    fn from(err: Error) -> Self {
        CodecError::IO(err)
    }
}

pub struct GameClientCodec {}

impl Decoder for GameClientCodec {
    type Item = ServerMessage;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<ServerMessage>, CodecError> {
        if src.len() == 0 {
            return Ok(None);
        }

        // Read message type.
        let message_type = src.get_u32();

        let message = match message_type {
            1 => ServerMessage::HelloFromServer {
                server_version: src.get_u32(),
            },

            _ => return Err(CodecError::InvalidMessageType),
        };

        Ok(Some(message))
    }
}

impl Encoder<ClientMessage> for GameClientCodec {
    type Error = CodecError;

    fn encode(&mut self, item: ClientMessage, dst: &mut BytesMut) -> Result<(), CodecError> {
        Ok(match item {
            ClientMessage::HelloFromClient { client_version } => {
                dst.put_u32(1);
                dst.put_u32(client_version);
            }

            ClientMessage::Pong => {
                dst.put_u32(2);
            }
        })
    }
}
