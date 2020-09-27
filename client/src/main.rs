use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8000".to_string();
    let mut stream = TcpStream::connect(&addr).await?;

    let mut bytes = BytesMut::new();
    bytes.reserve(8);
    bytes.put_u32(1);
    bytes.put_u32(9);
    stream.write_buf(&mut bytes).await?;

    let mut buf: [u8; 4096] = [0; 4096];
    let p = stream.read(&mut buf).await?;

    println!("read {} bytes", p);

    let mut buf = Bytes::copy_from_slice(&buf);
    println!("receive buffer length: {}", buf.len());
    let v1 = buf.get_u32();
    let v2 = buf.get_u32();
    println!("values: {} {}", v1, v2);

    Ok(())
}
