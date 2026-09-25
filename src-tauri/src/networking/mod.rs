//! Reserved for the future host/client networking modes.
//! Version 1 intentionally contains no server, WebSocket, or port mapping code.

#[allow(dead_code)]
pub enum RunMode {
    Local,
    NetworkHost,
    NetworkClient,
}
