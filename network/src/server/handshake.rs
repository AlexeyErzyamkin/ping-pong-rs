use tokio::{
    prelude::*,
    net::TcpStream,
    io
};

use futures::future::ok;

use crate::handshake::{HandshakeResult, HANDSHAKE_LEN, HANDSHAKE_STR, HANDSHAKE_OK, HANDSHAKE_FAIL};

pub fn new(stream: TcpStream) -> impl Future<Item = (TcpStream, HandshakeResult), Error = io::Error> {
    io::read_exact(stream, vec![0; HANDSHAKE_LEN])
        .and_then(|(stream, buf)| {
            let (result, response) = if buf == HANDSHAKE_STR.as_bytes() {
                (HandshakeResult::Ok, HANDSHAKE_OK)
            } else {
                (HandshakeResult::Failed, HANDSHAKE_FAIL)
            };

            let next = io::write_all(stream, response)
                .join(ok(result));

            next
        })
        .map(|((stream, _), result)| {
            (stream, result)
        })
}