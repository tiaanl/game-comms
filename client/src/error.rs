use crate::codec::CodecError;
use futures::io::Error;

#[derive(Debug)]
pub enum ClientError {
    IO(std::io::Error),
    CodecError(crate::codec::CodecError),
}

impl From<std::io::Error> for ClientError {
    fn from(err: Error) -> Self {
        ClientError::IO(err)
    }
}

impl From<crate::codec::CodecError> for ClientError {
    fn from(err: CodecError) -> Self {
        ClientError::CodecError(err)
    }
}
