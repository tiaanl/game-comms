pub enum ServerMessage {
    HelloFromServer { server_version: u32 },
    Ping,
}

pub enum ClientMessage {
    HelloFromClient { client_version: u32 },
    Pong,
}
