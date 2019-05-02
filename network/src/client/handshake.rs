use tokio::{
    prelude::*,
    net::TcpStream,
    io
};

use crate::handshake::{HandshakeResult, HANDSHAKE_STR, HANDSHAKE_OK, HANDSHAKE_RESULT_LEN};

pub fn new(stream: TcpStream) -> impl Future<Item = (TcpStream, HandshakeResult), Error = io::Error> {
    io::write_all(stream, HANDSHAKE_STR)
        .and_then(|(stream, _)| {
            io::read_exact(stream, vec![0; HANDSHAKE_RESULT_LEN])
        })
        .map(|(stream, buf)| {
            let result = if buf == HANDSHAKE_OK.as_bytes() {
                HandshakeResult::Ok
            } else {
                HandshakeResult::Failed
            };

            (stream, result)
        })
}