mod client_handler;
mod codec;
mod error;

use tokio::net::TcpListener;

use crate::client_handler::ClientHandler;
use crate::error::ServerError;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("server=info".parse()?))
        .with_span_events(FmtSpan::FULL)
        .init();

    let addr = "127.0.0.1:8000".to_string();
    let mut listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening for connections on tcp://{}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;

        tracing::debug!("Accepted connection from: {}", &addr);

        let mut client = ClientHandler::new(stream);

        tokio::spawn(async move {
            if let Err(err) = client.run().await {
                return Err(err);
            };

            Ok(())
        });
    }
}
