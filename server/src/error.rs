#[derive(Debug)]
pub enum ServerError {
    CodecError(crate::codec::CodecError),
    // LinesCodecError(LinesCodecError),
    IO(std::io::Error),
    ParseError(tracing_subscriber::filter::ParseError),
}

impl From<crate::codec::CodecError> for ServerError {
    fn from(err: crate::codec::CodecError) -> Self {
        ServerError::CodecError(err)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::IO(error)
    }
}

// impl From<LinesCodecError> for ServerError {
//     fn from(error: LinesCodecError) -> Self {
//         ServerError::LinesCodecError(error)
//     }
// }

impl From<tracing_subscriber::filter::ParseError> for ServerError {
    fn from(error: tracing_subscriber::filter::ParseError) -> Self {
        ServerError::ParseError(error)
    }
}
